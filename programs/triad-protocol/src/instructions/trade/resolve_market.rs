use anchor_lang::prelude::*;

use crate::{
    constraints::is_admin,
    errors::TriadProtocolError,
    events::MarketUpdate,
    MarketV2,
    WinningDirection,
};

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(mut, constraint = is_admin(&signer)?)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market: Box<Account<'info, MarketV2>>,

    pub system_program: Program<'info, System>,
}

pub fn resolve_market(
    ctx: Context<ResolveMarket>,
    winning_direction: WinningDirection
) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let current_timestamp = Clock::get()?.unix_timestamp;

    market.is_active = false;

    require!(
        market.winning_direction == WinningDirection::None,
        TriadProtocolError::MarketAlreadyResolved
    );

    market.winning_direction = winning_direction;

    emit!(MarketUpdate {
        market_id: market.market_id,
        question: String::from_utf8_lossy(&market.question).to_string(),
        start_time: market.market_start,
        end_time: market.market_end,
        hype_liquidity: market.hype_liquidity,
        flop_liquidity: market.flop_liquidity,
        winning_direction: market.winning_direction,
        final_hype_price: market.hype_price,
        final_flop_price: market.flop_price,
        timestamp: current_timestamp,
        total_hype_shares: market.hype_shares,
        total_flop_shares: market.flop_shares,
    });

    Ok(())
}
