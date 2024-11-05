use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{ state::{ MarketV2, InitializeMarketArgs }, constraints::is_admin };

#[derive(Accounts)]
#[instruction(args: InitializeMarketArgs)]
pub struct InitializeMarket<'info> {
    #[account(mut, constraint = is_admin(&signer)?)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = MarketV2::SPACE,
        seeds = [MarketV2::PREFIX_SEED, &args.market_id.to_le_bytes()],
        bump
    )]
    pub market: Box<Account<'info, MarketV2>>,

    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_market(ctx: Context<InitializeMarket>, args: InitializeMarketArgs) -> Result<()> {
    let market = &mut ctx.accounts.market;

    let ts = Clock::get()?.unix_timestamp;

    market.set_inner(MarketV2 {
        bump: ctx.bumps.market,
        authority: ctx.accounts.signer.key(),
        market_id: args.market_id,
        mint: ctx.accounts.mint.key(),
        market_start: args.start_time,
        market_end: args.end_time,
        question: args.question,
        update_ts: ts,
        ..Default::default()
    });

    Ok(())
}
