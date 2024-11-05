use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{ state::MarketV2, errors::TriadProtocolError };

#[derive(Accounts)]
pub struct CollectFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, constraint = market.authority == signer.key())]
    pub market: Box<Account<'info, MarketV2>>,

    #[account(mut, constraint = mint.key() == market.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_ata: Box<InterfaceAccount<'info, TokenAccount>>,

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
    let market = &mut ctx.accounts.market;

    let market_available = market.market_fee_available
        .checked_sub(market.market_fee_claimed)
        .unwrap();
    let holder_available = market.nft_holders_fee_available
        .checked_sub(market.nft_holders_fee_claimed)
        .unwrap();

    let amount = market_available.checked_add(holder_available).unwrap();

    require!(amount > 0, TriadProtocolError::InsufficientFunds);

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.market_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.signer_ata.to_account_info(),
                authority: market.to_account_info(),
            },
            &[&[b"market", &market.market_id.to_le_bytes(), &[market.bump]]]
        ),
        amount,
        ctx.accounts.mint.decimals
    )?;

    market.market_fee_claimed = market.market_fee_claimed.checked_add(market_available).unwrap();
    market.nft_holders_fee_claimed = market.nft_holders_fee_claimed
        .checked_add(holder_available)
        .unwrap();

    Ok(())
}
