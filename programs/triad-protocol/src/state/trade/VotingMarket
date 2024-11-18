use anchor_lang::prelude::*;
// based on Market_v2
// 
#[program]
pub mod voting_market_program {
    use super::*;

    pub fn create_voting_market(ctx: Context<CreateVotingMarket>, question: String) -> Result<()> {
        let fee = 0.05 * 1_000_000_000; //  Fee 0.05 SOL to initiate new vote_market
        let user = &ctx.accounts.authority; // 
        let fee_recipient = &ctx.accounts.fee_recipient; // account that will receive the Fee

        // Check if there is enough balance
        if **user.lamports.borrow() < fee as u64 {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Deduct the user's fee and send it to the destination account
        **user.lamports.borrow_mut() -= fee as u64;
        **fee_recipient.lamports.borrow_mut() += fee as u64;

        let vote_id = ctx.accounts.voting_market.vote_id;
        let data_inicio = Clock::get()?.unix_timestamp; // initial vote date is actual timestamp
        let data_fim = data_inicio + 7 * 24 * 60 * 60; // end vote date is 7 days after initial date
        let data_inicio_market = data_fim + 24 * 60 * 60; // initial market date is 1 day after end vote date
        let data_close = data_inicio_market + 6 * 24 * 60 * 60; // close market date is 6 days after initial market date
        let data_fim_market = data_close + 24 * 60 * 60; // resolve market date is 1 day after close market date
        
        let voting_market = &mut ctx.accounts.voting_market;
        voting_market.vote_id = vote_id;
        voting_market.question = question.as_bytes().try_into().unwrap(); // save the question
        voting_market.local_de_verificacao = ctx.accounts.local_de_verificacao.clone();
        voting_market.tipo_mercado = "default".to_string(); // Tipo de mercado inicial
        voting_market.data_inicio = data_inicio;
        voting_market.data_fim = data_fim;
        voting_market.data_inicio_market = data_inicio_market;
        voting_market.data_close = data_close;
        voting_market.data_fim_market = data_fim_market;
        voting_market.total_hype = 1;
        voting_market.total_flop = 0;
        voting_market.min_votes = 1000;

        Ok(())
    }
}

// account definition
#[derive(Accounts)]
pub struct CreateVotingMarket<'info> {
    #[account(
        init, 
        payer = authority, 
        space = VotingMarket::SPACE
    )]
    pub voting_market: Account<'info, VotingMarket>,
    
    #[account(mut)]
    pub authority: Signer<'info>, // account that starts vote
    
    pub local_de_verificacao: String, // 
    
    #[account(mut)] // fee vault
    pub fee_recipient: AccountInfo<'info>, 
    
    pub system_program: Program<'info, System>,
}

// Struct VotingMarket
#[account]
pub struct VotingMarket {
    pub bump: u8,
    pub authority: Pubkey, // 
    pub vote_id: u64, // voteID
    pub question: [u8; 80],
    pub local_de_verificacao: String, 
    pub tipo_mercado: String, // 
    pub data_inicio: i64, // 
    pub data_fim: i64, // Data de fim
    pub data_inicio_market: i64, // 
    pub data_close: i64, //
    pub data_fim_market: i64, //
    pub total_hype: u64, // total hype votes
    pub total_flop: u64, // total flop votes
    pub min_votes: u64, // min votes
    pub padding: [u8; 64], // 
}

impl VotingMarket {
    pub const PREFIX_SEED: &'static [u8] = b"voting_market";
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>(); // 
}

// error
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to pay the fee.")]
    InsufficientFunds,
}
