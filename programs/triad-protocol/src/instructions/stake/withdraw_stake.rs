use crate::constants::{ADMIN, TTRIAD_MINT};
use crate::constraints::is_authority_for_stake;
use crate::NFTRewards;
use crate::{errors::TriadProtocolError, state::Stake, StakeVault};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{close_account, CloseAccount, Token2022};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};
use std::str::FromStr;

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub stake_vault: Box<Account<'info, StakeVault>>,

    #[account(mut, close = signer, constraint = is_authority_for_stake(&stake, &signer)?)]
    pub stake: Box<Account<'info, Stake>>,

    /// CHECK: Just Admin account the recovery the rent
    #[account(mut, constraint = admin.key.to_string() == ADMIN)]
    pub admin: AccountInfo<'info>,

    #[account(mut, close = admin)]
    pub nft_rewards: Account<'info, NFTRewards>,

    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub from_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        constraint = signer.key() == to_ata.owner && to_ata.mint == mint.key()
    )]
    pub to_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
    let mint = &ctx.accounts.mint.to_account_info();
    let stake = &mut ctx.accounts.stake;
    let stake_vault = &mut ctx.accounts.stake_vault;

    if stake.mint != *mint.key {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    if stake.withdraw_ts > Clock::get()?.unix_timestamp {
        return Err(TriadProtocolError::StakeLocked.into());
    }

    let signer: &[&[&[u8]]] = &[&[
        b"stake_vault",
        stake_vault.name.as_bytes(),
        &[stake_vault.bump],
    ]];

    let is_token_stake = stake.mint.eq(&Pubkey::from_str(TTRIAD_MINT).unwrap());

    let mut amount = 1;

    if is_token_stake {
        amount = stake.amount;
    }

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.from_ata.to_account_info().clone(),
            mint: ctx.accounts.mint.to_account_info().clone(),
            to: ctx.accounts.to_ata.to_account_info().clone(),
            authority: stake_vault.to_account_info(),
        },
        signer,
    );

    transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;

    if is_token_stake {
        stake_vault.token_staked -= stake.amount;
    } else {
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.from_ata.to_account_info(),
                destination: ctx.accounts.signer.to_account_info(),
                authority: stake_vault.to_account_info(),
            },
            signer,
        ))?;

        stake_vault.nft_staked -= 1;
    }

    Ok(())
}
