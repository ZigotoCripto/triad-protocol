use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{
    constraints::is_mint_for_stake_vault,
    errors::TriadProtocolError,
    state::{ StakeTokenArgs, StakeVault, StakeV2, User },
};

#[derive(Accounts)]
#[instruction(args: StakeTokenArgs)]
pub struct StakeToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [StakeVault::PREFIX_SEED, args.stake_vault.as_bytes()], bump)]
    pub stake_vault: Box<Account<'info, StakeVault>>,

    #[account(mut, constraint = user.authority == *signer.key)]
    pub user: Box<Account<'info, User>>,

    #[account(
        init,
        payer = signer,
        space = StakeV2::SPACE,
        seeds = [StakeV2::PREFIX_SEED, signer.key().as_ref(), args.name.as_bytes()],
        bump
    )]
    pub stake: Box<Account<'info, StakeV2>>,

    #[account(mut, constraint = is_mint_for_stake_vault(&stake_vault, &mint.key())?)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut, 
        constraint = from_ata.amount >= args.amount,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub from_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = stake_vault,
        associated_token::token_program = token_program
    )]
    pub to_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn stake_token(ctx: Context<StakeToken>, args: StakeTokenArgs) -> Result<()> {
    let mint = &ctx.accounts.mint.to_account_info();
    let stake = &mut ctx.accounts.stake;
    let stake_vault = &mut ctx.accounts.stake_vault;
    let user = &mut ctx.accounts.user;

    if stake_vault.is_locked {
        return Err(TriadProtocolError::StakeVaultLocked.into());
    }

    stake.bump = ctx.bumps.stake;
    stake.authority = *ctx.accounts.signer.key;
    stake.init_ts = Clock::get()?.unix_timestamp;
    stake.withdraw_ts = 0;
    stake.claimed_ts = 0;
    stake.name = args.name;
    stake.mint = *mint.key;
    stake.boost = false;
    stake.stake_vault = stake_vault.key();
    stake.claimed = 0;
    stake.available = 0;
    stake.amount = args.amount;

    stake_vault.token_staked = stake_vault.token_staked.checked_add(args.amount).unwrap();

    user.staked = user.staked.checked_add(args.amount).unwrap();

    transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked {
            from: ctx.accounts.from_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }),
        args.amount,
        ctx.accounts.mint.decimals
    )?;

    Ok(())
}
