use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{
    state::{ MarketV2, UserTrade, OrderStatus, OrderDirection, WinningDirection },
    errors::TriadProtocolError,
    events::OrderUpdate,
    constraints::is_authority_for_user_trade,
};

#[derive(Accounts)]
pub struct PayoutOrder<'info> {
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
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn payout_order(ctx: Context<PayoutOrder>, order_id: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let user_trade = &mut ctx.accounts.user_trade;

    require!(
        market.winning_direction != WinningDirection::None,
        TriadProtocolError::MarketNotResolved
    );

    let order_index = user_trade.orders
        .iter()
        .position(|order| order.order_id == order_id && order.status == OrderStatus::Open)
        .ok_or(TriadProtocolError::OrderNotFound)?;

    let order = user_trade.orders[order_index];

    require!(market.market_id == order.market_id, TriadProtocolError::OrderNotOpen);

    let (shares, is_winner) = match (order.direction, market.winning_direction) {
        | (OrderDirection::Hype, WinningDirection::Hype)
        | (OrderDirection::Flop, WinningDirection::Flop) => {
            let winning_payout = order.total_shares;
            (winning_payout, true)
        }
        _ => { (0, false) }
    };

    let (market_shares, market_opposit_liquidity, market_liquidity) = match
        market.winning_direction
    {
        WinningDirection::Hype =>
            (market.hype_shares, market.flop_liquidity, market.hype_liquidity),
        WinningDirection::Flop =>
            (market.flop_shares, market.hype_liquidity, market.flop_liquidity),
        _ => (0, 0, 0),
    };

    let mut med_price = 1.0;

    let market_liquidity_at_start = if market.market_liquidity_at_start == 0 {
        1_000_000_000
    } else {
        market.market_liquidity_at_start
    };

    let markets_liquidity = market_liquidity
        .checked_add(market_opposit_liquidity)
        .unwrap()
        .checked_sub(market_liquidity_at_start)
        .unwrap();

    if market_shares > markets_liquidity {
        med_price = (markets_liquidity as f64) / (market_shares as f64);
    }

    msg!("Med Price {:?}", med_price);
    msg!("Market Shares {:?}", market_shares);
    msg!("Markets Liquidity {:?}", markets_liquidity);
    msg!("Initial Liquidity {:?}", market_liquidity_at_start);
    msg!("Order Shares {:?}", order.total_shares);
    msg!("Is Winner {:?}", is_winner);
    msg!("Order Amount {:?}", order.total_amount);

    let payout = if !is_winner {
        0
    } else {
        let float_payout =
            ((shares as f64) - (order.total_amount as f64)) * med_price +
            (order.total_amount as f64);
        float_payout.round() as u64
    };

    if payout > 0 && is_winner {
        let signer: &[&[&[u8]]] = &[&[b"market", &market.market_id.to_le_bytes(), &[market.bump]]];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.market_vault.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    authority: market.to_account_info(),
                },
                signer
            ),
            payout,
            ctx.accounts.mint.decimals
        )?;

        user_trade.total_withdraws = user_trade.total_withdraws.checked_add(payout).unwrap();

        msg!("Amount {:?}", order.total_amount);
        msg!("Market Shares {:?}", market_shares);
        msg!("Market Opposit Liquidity {:?}", market_opposit_liquidity);
        msg!("Med Price {:?}", med_price);
        msg!("Payout {:?}", payout);
    }

    let pnl = (payout as i64) - (order.total_amount as i64);

    user_trade.orders[order_index].status = OrderStatus::Closed;
    user_trade.opened_orders = user_trade.opened_orders.checked_sub(1).unwrap();

    market.opened_orders = market.opened_orders.checked_sub(1).unwrap();

    emit!(OrderUpdate {
        user: *ctx.accounts.signer.key,
        market_id: market.market_id,
        order_id: order.order_id,
        direction: order.direction,
        order_type: order.order_type,
        question_id: order.question_id,
        order_status: OrderStatus::Closed,
        price: order.price,
        total_shares: order.total_shares,
        total_amount: order.total_amount,
        refund_amount: Some(payout),
        timestamp: Clock::get()?.unix_timestamp,
        is_question_winner: Some(is_winner),
        pnl,
    });

    Ok(())
}
