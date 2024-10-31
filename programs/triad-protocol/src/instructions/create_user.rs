use anchor_lang::prelude::*;

use crate::{ state::User, CreateUserArgs };

#[derive(Accounts)]
#[instruction(args: CreateUserArgs)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub referral: Box<Account<'info, User>>,

    #[account(
        init,
        payer = signer,
        space = User::SPACE,
        seeds = [User::PREFIX_SEED, signer.key().as_ref()],
        bump
    )]
    pub user: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>, args: CreateUserArgs) -> Result<()> {
    let user: &mut Account<User> = &mut ctx.accounts.user;
    let referral: &mut Account<User> = &mut ctx.accounts.referral;

    user.set_inner(User {
        ts: Clock::get()?.unix_timestamp,
        bump: ctx.bumps.user,
        authority: *ctx.accounts.signer.key,
        referral: referral.key(),
        referred: 0,
        name: args.name,
        swaps: 0,
        staked: 0,
        swaps_made: 0,
        first_swap: 0,
        user_trade: Pubkey::default(),
    });

    referral.referred += 1;

    Ok(())
}
