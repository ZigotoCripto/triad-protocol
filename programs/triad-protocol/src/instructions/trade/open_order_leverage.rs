use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{
    state::{ MarketV2, UserTrade, Order, OrderDirection, OrderStatus, OrderType, OpenOrderArgs },
    errors::TriadProtocolError,
    events::OrderUpdate,
    constraints::is_authority_for_user_trade,
};

#[derive(Accounts)]
#[instruction(args: OpenOrderArgs)]
pub struct OpenOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = is_authority_for_user_trade(&user_trade, &signer)?
    )]
    pub user_trade: Box<Account<'info, UserTrade>>,

    #[account(mut)]
    pub market: Box<Account<'info, MarketV2>>,

    #[account(mut, constraint = mint.key() == market.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut, 
        constraint = user_from_ata.amount >= args.amount,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_from_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_to_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn open_order(ctx: Context<OpenOrder>, args: OpenOrderArgs) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let user_trade = &mut ctx.accounts.user_trade;
    let leverage = args.leverage //get leverage, 1(normal) or 2(double)
    let ts = Clock::get()?.unix_timestamp;

    require!(market.is_active, TriadProtocolError::MarketInactive);
    require!(ts > market.market_start, TriadProtocolError::QuestionPeriodNotStarted);
    require!(market.market_end > ts, TriadProtocolError::QuestionPeriodEnded);

    let (current_price, current_liquidity, otherside_current_liquidity) = match args.direction {
        OrderDirection::Hype => (market.hype_price, market.hype_liquidity, market.flop_liquidity),
        OrderDirection::Flop => (market.flop_price, market.flop_liquidity, market.hype_liquidity),
    };

    require!(ts > market.update_ts, TriadProtocolError::ConcurrentTransaction);
    require!(current_price > 0, TriadProtocolError::InvalidPrice);
    require!(current_liquidity > 0, TriadProtocolError::InsufficientLiquidity);
    require!(otherside_current_liquidity > 0, TriadProtocolError::InsufficientLiquidity);

    let fee_amount = ((args.amount * (market.fee_bps as u64)) / 100000) as u64;
    let net_amount = args.amount.saturating_sub(fee_amount);
    let net_amount_leverage = net_amount.checked_mul(leverage).unwrap(); // setting new leveraged amount
    require!(net_amount > current_price, TriadProtocolError::InsufficientFunds);
    
    let new_directional_liquidity = current_liquidity.checked_add(net_amount_leverage).unwrap();
    let markets_liquidity = new_directional_liquidity
        .checked_add(otherside_current_liquidity)
        .unwrap();

    let new_price = new_directional_liquidity
        .checked_mul(1_000_000)
        .unwrap()
        .checked_div(markets_liquidity)
        .unwrap()
        .clamp(1, 999_999);

    let total_shares = net_amount_leverage.checked_mul(1_000_000).unwrap().checked_div(new_price).unwrap();

    if total_shares.eq(&0) {
        return Err(TriadProtocolError::InsufficientFunds.into());
    }

    let order_index = user_trade.orders
        .iter()
        .position(|order| order.status != OrderStatus::Open)
        .ok_or(TriadProtocolError::NoAvailableOrderSlot)?;

    user_trade.orders[order_index] = Order {
        ts,
        order_id: market.next_order_id(),
        question_id: 0,
        market_id: market.market_id,
        status: OrderStatus::Open,
        price: new_price,
        total_amount: net_amount,
        total_shares,
        order_type: OrderType::Market,
        direction: args.direction,
        padding: [0; 32],
        leverage: args.leverage,
    };

    user_trade.opened_orders = user_trade.opened_orders.checked_add(1).unwrap();
    user_trade.total_deposits = user_trade.total_deposits.checked_add(net_amount).unwrap();

    market.opened_orders = market.opened_orders.checked_add(1).unwrap();
    market.volume = market.volume.checked_add(net_amount_leverage).unwrap();
    market.update_ts = ts;

    // Update market shares
    match args.direction {
        OrderDirection::Hype => {
            market.hype_shares = market.hype_shares.checked_add(total_shares).unwrap();
        }
        OrderDirection::Flop => {
            market.flop_shares = market.flop_shares.checked_add(total_shares).unwrap();
        }
    }

    market.update_price(net_amount, new_price, args.direction, true)?;

    // Calculate fee distribution
    let nft_holders_fee = (fee_amount * 100) / 10000; // 0.1%
    let market_fee = fee_amount - nft_holders_fee; // Remaining 2% fee

    market.nft_holders_fee_available = market.nft_holders_fee_available
        .checked_add(nft_holders_fee)
        .unwrap();
    market.market_fee_available = market.market_fee_available.checked_add(market_fee).unwrap();

    transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked {
            from: ctx.accounts.user_from_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.market_to_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }),
        net_amount.checked_add(fee_amount).unwrap(),
        ctx.accounts.mint.decimals
    )?;

    let current_order = user_trade.orders[order_index];

    emit!(OrderUpdate {
        timestamp: current_order.ts,
        user: user_trade.authority,
        question_id: current_order.question_id,
        market_id: current_order.market_id,
        order_id: current_order.order_id,
        direction: current_order.direction,
        order_type: current_order.order_type,
        order_status: current_order.status,
        total_shares: current_order.total_shares,
        total_amount: current_order.total_amount,
        pnl: 0,
        price: current_order.price,
        refund_amount: None,
        is_question_winner: None,
    });

    Ok(())
}
