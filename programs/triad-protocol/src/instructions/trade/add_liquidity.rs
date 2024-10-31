use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ Token2022, transfer_checked, TransferChecked };
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };

use crate::{
    state::{ Market, OrderDirection, OpenOrderArgs, AddLiquidityArgs },
    errors::TriadProtocolError,
};

#[derive(Accounts)]
#[instruction(args: OpenOrderArgs)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, constraint = market.authority == signer.key())]
    pub market: Box<Account<'info, Market>>,

    #[account(mut, constraint = mint.key() == market.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut, 
        constraint = user_from_ata.amount >= args.amount,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_from_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_to_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, args: AddLiquidityArgs) -> Result<()> {
    let market = &mut ctx.accounts.market;

    require!(!market.is_active, TriadProtocolError::MarketInactive);

    match args.direction {
        OrderDirection::Hype => {
            market.hype_liquidity = market.hype_liquidity.checked_add(args.amount).unwrap();
        }
        OrderDirection::Flop => {
            market.flop_liquidity = market.flop_liquidity.checked_add(args.amount).unwrap();
        }
    }

    transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked {
            from: ctx.accounts.user_from_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.market_to_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }),
        args.amount,
        ctx.accounts.mint.decimals
    )?;

    Ok(())
}
