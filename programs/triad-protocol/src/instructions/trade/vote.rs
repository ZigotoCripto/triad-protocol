use anchor_lang::prelude::*;
use anchor_spl::token_2022::{Token2022, transfer_checked, TransferChecked};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount},
};
//based on open_order
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>, // Voter's account

    #[account(mut)]
    pub voting_market: Account<'info, VotingMarket>, // Voting market

    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>, // Currency used for paid votes

    #[account(
        mut,
        constraint = voter_ata.amount >= amount,
        associated_token::mint = mint,
        associated_token::authority = voter,
        associated_token::token_program = token_program
    )]
    pub voter_ata: Box<InterfaceAccount<'info, TokenAccount>>, // Voter's token account

    #[account(
        init_if_needed,
        payer = voter,
        associated_token::mint = mint,
        associated_token::authority = voting_market,
        associated_token::token_program = token_program
    )]
    pub market_ata: Box<InterfaceAccount<'info, TokenAccount>>, // Market's token account

    pub token_program: Program<'info, Token2022>, // Token program
    pub associated_token_program: Program<'info, AssociatedToken>, // Associated accounts program
    pub system_program: Program<'info, System>, // System program
}

pub fn vote(ctx: Context<Vote>, direction: VoteDirection, amount: u64) -> Result<()> {
    let voting_market = &mut ctx.accounts.voting_market;
    let voter_key = ctx.accounts.voter.key();

    // Validate voting period
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= voting_market.data_inicio && now <= voting_market.data_fim,
        CustomError::VotingNotActive
    );

    // Check if the voter has already participated
    let is_first_vote = !voting_market
        .voter_map
        .iter()
        .any(|v| *v == voter_key);

    if is_first_vote {
        // First vote is free
        voting_market.voter_map.push(voter_key);
    } else {
        // Charge for additional votes
        require!(amount > 0, CustomError::InvalidAmount);

        transfer_checked(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked {
                from: ctx.accounts.voter_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.market_ata.to_account_info(),
                authority: ctx.accounts.voter.to_account_info(),
            }),
            amount,
            ctx.accounts.mint.decimals,
        )?;
    }

    // Update votes in the market
    match direction {
        VoteDirection::Hype => {
            voting_market.total_hype += 1 + amount; // Includes paid votes (if applicable)
        }
        VoteDirection::Flop => {
            voting_market.total_flop += 1 + amount; // Includes paid votes (if applicable)
        }
    }

    Ok(())
}

// Enum for vote direction
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteDirection {
    Hype,
    Flop,
}

// Update VotingMarket struct
#[account]
pub struct VotingMarket {
    pub authority: Pubkey,       // Authority that created the market
    pub vote_id: u64,            // Unique identifier for the voting session
    pub question: [u8; 80],      // Voting question
    pub data_inicio: i64,        // Start date of the voting session
    pub data_fim: i64,           // End date of the voting session
    pub total_hype: u64,         // Total Hype votes
    pub total_flop: u64,         // Total Flop votes
    pub voter_map: Vec<Pubkey>,  // List of voters who participated
}

// Definition of custom errors
#[error_code]
pub enum CustomError {
    #[msg("Voting is not active.")]
    VotingNotActive,
    #[msg("Invalid amount for paid votes.")]
    InvalidAmount,
}
