use crate::{state::Vault, Ticker};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction()]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub ticker: Account<'info, Ticker>,

    #[account(init, payer = signer, space = Vault::SPACE, seeds = [Vault::PREFIX_SEED.as_ref(), ticker.name.as_ref()], bump)]
    pub vault: Account<'info, Vault>,

    pub payer_token_mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [Vault::PREFIX_SEED_VAULT_TOKEN_ACCOUNT.as_ref(), vault.key().as_ref()],
        bump,
        payer = signer,
        token::mint = payer_token_mint,
        token::authority = vault,
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.bump = *ctx.bumps.get("vault").unwrap();
    vault.authority = *ctx.accounts.signer.key;
    vault.name = ctx.accounts.ticker.name.clone();
    vault.ticker_address = *ctx.accounts.ticker.to_account_info().key;
    vault.token_account = *ctx.accounts.token_account.to_account_info().key;
    vault.total_deposits = 0;
    vault.total_withdraws = 0;
    vault.init_ts = Clock::get()?.unix_timestamp;
    vault.net_deposits = 0;

    Ok(())
}
