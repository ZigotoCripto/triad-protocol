use anchor_lang::prelude::*;

use crate::{ state::{ User, UserTrade, Order }, constraints::is_authority_for_user };

#[derive(Accounts)]
pub struct CreateUserTrade<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, constraint = is_authority_for_user(&user, &signer)?)]
    pub user: Box<Account<'info, User>>,

    #[account(
        init,
        payer = signer,
        space = UserTrade::SPACE,
        seeds = [UserTrade::PREFIX_SEED, signer.key().as_ref()],
        bump
    )]
    pub user_trade: Box<Account<'info, UserTrade>>,

    pub system_program: Program<'info, System>,
}

pub fn create_user_trade(ctx: Context<CreateUserTrade>) -> Result<()> {
    let user_trade = &mut ctx.accounts.user_trade;
    let user = &mut ctx.accounts.user;

    user_trade.set_inner(UserTrade {
        bump: ctx.bumps.user_trade,
        authority: ctx.accounts.signer.key(),
        total_deposits: 0,
        total_withdraws: 0,
        opened_orders: 0,
        orders: [Order::default(); 10],
        padding: [0; 32],
    });

    user.user_trade = user_trade.key();

    Ok(())
}
