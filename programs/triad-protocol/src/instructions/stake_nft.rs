use std::str::FromStr;

use crate::{
    constants::{MYSTERY_BOX_PROGRAM, TRIAD_MYSTERY_BOX}, errors::TriadProtocolError, state::{Stake, StakeNFTArgs}, StakeVault
};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::BaseStateWithExtensions;
use anchor_spl::token_2022::{
    spl_token_2022::{extension::PodStateWithExtensions, pod::PodMint},
    Token2022,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};
use spl_token_metadata_interface::state::TokenMetadata;

#[derive(Accounts)]
#[instruction(args: StakeNFTArgs)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [StakeVault::PREFIX_SEED, args.name.as_bytes()], bump)]
    pub stake_vault: Box<Account<'info, StakeVault>>,

    #[account(init, payer = signer, space = Stake::SPACE, seeds = [Stake::PREFIX_SEED, mint.key().as_ref()], bump)]
    pub stake: Box<Account<'info, Stake>>,

    #[account(
        mut,
        extensions::metadata_pointer::metadata_address = mint,
        seeds = [b"mint", args.name.as_bytes()], bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        constraint = from_ata.amount >= 1,
    )]
    pub from_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        associated_token::mint = mint,
        payer = signer,
        associated_token::authority = stake
    )]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn stake_nft(ctx: Context<StakeNFT>, args: StakeNFTArgs) -> Result<()> {
    if ctx.accounts.signer.key() != ctx.accounts.from_ata.to_account_info().owner.key() {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    let mint = &ctx.accounts.mint.to_account_info();
    let buffer = mint.try_borrow_data()?;
    let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;
    let token_metadata = state.get_variable_len_extension::<TokenMetadata>()?;

    if token_metadata.update_authority.0 != Pubkey::from_str(TRIAD_MYSTERY_BOX).unwrap() {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    let (mint_seed, _bump) = Pubkey::find_program_address(&[b"mint", args.name.as_bytes()], &Pubkey::from_str(MYSTERY_BOX_PROGRAM).unwrap());

    if mint_seed != mint.key() {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    let stake = &mut ctx.accounts.stake;

    stake.authority = *ctx.accounts.signer.key;
    stake.init_ts = Clock::get()?.unix_timestamp;
    stake.is_locked = true;
    stake.collections = args.collections;
    stake.bump = ctx.bumps.stake;
    stake.rarity = args.rarity;
    stake.from_ata = *ctx.accounts.from_ata.to_account_info().key;
    stake.name = token_metadata.name;
    stake.stake_vault = *ctx.accounts.stake_vault.to_account_info().key;

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.from_ata.to_account_info().clone(),
        mint: ctx.accounts.mint.to_account_info().clone(),
        to: ctx.accounts.to_ata.to_account_info().clone(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_context, 1, ctx.accounts.mint.decimals)?;

    Ok(())
}
