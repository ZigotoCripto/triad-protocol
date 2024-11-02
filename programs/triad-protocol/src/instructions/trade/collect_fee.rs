use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{ state::{ Market, FeeVault }, errors::TriadProtocolError };

#[derive(Accounts)]
pub struct CollectFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, constraint = market.authority == signer.key())]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        constraint = fee_vault.market == market.key(),
        constraint = fee_vault.authority == signer.key()
    )]
    pub fee_vault: Box<Account<'info, FeeVault>>,

    #[account(mut, constraint = mint.key() == market.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = fee_vault,
        associated_token::token_program = token_program
    )]
    pub fee_vault_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub signer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn collect_fee(ctx: Context<CollectFee>) -> Result<()> {
    let market = &ctx.accounts.market;
    let fee_vault = &mut ctx.accounts.fee_vault;

    let market_available = fee_vault.market_available
        .checked_sub(fee_vault.market_claimed)
        .unwrap();
    let project_available = fee_vault.project_available
        .checked_sub(fee_vault.project_claimed)
        .unwrap();
    let holder_available = fee_vault.nft_holders_available
        .checked_sub(fee_vault.nft_holders_claimed)
        .unwrap();

    let amount = market_available
        .checked_add(project_available)
        .unwrap()
        .checked_add(holder_available)
        .unwrap();

    require!(amount > 0, TriadProtocolError::InsufficientFunds);

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.fee_vault_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.signer_ata.to_account_info(),
                authority: fee_vault.to_account_info(),
            },
            &[&[b"fee_vault", &market.market_id.to_le_bytes(), &[fee_vault.bump]]]
        ),
        amount,
        ctx.accounts.mint.decimals
    )?;

    fee_vault.market_claimed = fee_vault.market_claimed.checked_add(market_available).unwrap();
    fee_vault.project_claimed = fee_vault.project_claimed.checked_add(project_available).unwrap();
    fee_vault.nft_holders_claimed = fee_vault.nft_holders_claimed
        .checked_add(holder_available)
        .unwrap();

    fee_vault.withdrawn = fee_vault.withdrawn.checked_add(amount).unwrap();
    fee_vault.net_balance = fee_vault.net_balance.checked_sub(amount).unwrap();

    Ok(())
}
