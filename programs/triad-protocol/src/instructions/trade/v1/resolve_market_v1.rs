use anchor_lang::prelude::*;

use crate::{ constraints::is_admin, errors::TriadProtocolError, Market, WinningDirection };

#[derive(Accounts)]
pub struct ResolveMarketV1<'info> {
    #[account(mut, constraint = is_admin(&signer)?)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market: Box<Account<'info, Market>>,

    pub system_program: Program<'info, System>,
}

pub fn resolve_market_v1(
    ctx: Context<ResolveMarketV1>,
    winning_direction: WinningDirection
) -> Result<()> {
    let market = &mut ctx.accounts.market;

    require!(
        market.previous_resolved_question.winning_direction == WinningDirection::None,
        TriadProtocolError::MarketAlreadyResolved
    );

    market.is_active = false;
    market.previous_resolved_question.winning_direction = winning_direction;

    Ok(())
}
