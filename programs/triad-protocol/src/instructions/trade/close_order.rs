use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{
    state::{ Market, UserTrade, OrderStatus, OrderDirection, Order },
    errors::TriadProtocolError,
    events::OrderUpdate,
    constraints::is_authority_for_user_trade,
};

#[derive(Accounts)]
pub struct CloseOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint = is_authority_for_user_trade(&user_trade, &signer)?
    )]
    pub user_trade: Box<Account<'info, UserTrade>>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

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

pub fn close_order(ctx: Context<CloseOrder>, order_id: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let user_trade = &mut ctx.accounts.user_trade;

    let ts = Clock::get()?.unix_timestamp;

    require!(market.is_active, TriadProtocolError::MarketInactive);
    require!(market.current_question_start > 0, TriadProtocolError::QuestionPeriodNotStarted);

    let order_index = user_trade.orders
        .iter()
        .position(|order| order.order_id == order_id && order.market_id == market.market_id)
        .ok_or(TriadProtocolError::OrderNotFound)?;

    let order = user_trade.orders[order_index];

    require!(ts > market.update_ts, TriadProtocolError::ConcurrentTransaction);
    require!(market.market_id == order.market_id, TriadProtocolError::Unauthorized);

    let (current_price, current_liquidity, otherside_current_liquidity) = match order.direction {
        OrderDirection::Hype => (market.hype_price, market.hype_liquidity, market.flop_liquidity),
        OrderDirection::Flop => (market.flop_price, market.flop_liquidity, market.hype_liquidity),
    };

    require!(current_liquidity > 0, TriadProtocolError::InsufficientLiquidity);

    let current_amount = (order.total_shares * current_price) / 1_000_000;

    let new_directional_liquidity = current_liquidity.checked_sub(current_amount).unwrap();
    let new_total_liquidity = new_directional_liquidity
        .checked_sub(otherside_current_liquidity)
        .unwrap();
    let mut new_price = ((new_directional_liquidity as f64) / (new_total_liquidity as f64)) as u64;

    new_price = new_price.clamp(1, 999_999);

    require!(current_liquidity > current_amount, TriadProtocolError::InsufficientLiquidity);

    if current_amount > 0 {
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
            current_amount,
            ctx.accounts.mint.decimals
        )?;

        market.update_price(current_amount, new_price, order.direction, None, false)?;
    }

    match order.direction {
        OrderDirection::Hype => {
            market.total_hype_shares = market.total_hype_shares
                .checked_sub(order.total_shares)
                .unwrap();
        }
        OrderDirection::Flop => {
            market.total_flop_shares = market.total_flop_shares
                .checked_sub(order.total_shares)
                .unwrap();
        }
    }

    user_trade.total_withdraws = user_trade.total_withdraws.checked_add(current_amount).unwrap();

    market.total_volume = market.total_volume.checked_add(current_amount).unwrap();
    market.open_orders_count = market.open_orders_count.checked_sub(1).unwrap();
    market.update_ts = ts;

    let total_amount = order.total_amount;

    user_trade.orders[order_index] = Order::default();

    emit!(OrderUpdate {
        user: *ctx.accounts.signer.key,
        market_id: market.market_id,
        order_id,
        direction: order.direction,
        order_type: order.order_type,
        question_id: order.question_id,
        order_status: OrderStatus::Closed,
        price: current_price,
        total_shares: order.total_shares,
        total_amount: order.total_amount,
        comment: None,
        refund_amount: Some(current_amount),
        timestamp: ts,
        is_question_winner: None,
        pnl: current_amount
            .checked_sub(total_amount)
            .map(|v| v as i64)
            .unwrap_or(-(total_amount as i64)),
    });

    Ok(())
}
