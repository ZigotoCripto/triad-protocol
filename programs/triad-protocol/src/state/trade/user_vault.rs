use anchor_lang::prelude::*;
// based on feevault
// Program definition
#[program]
pub mod user_vault_program {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        let fee = 0.02 * 1_000_000_000; // Vault creation fee (0.02 SOL)
        let user = &ctx.accounts.user; // Reference to the User
        let fee_recipient = &ctx.accounts.fee_recipient; // Reference to the account receiving the fees

        // Check if the user has enough balance to pay the fee
        if **user.lamports.borrow() < fee as u64 {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Deduct the fee from the user and send it to the destination account
        **user.lamports.borrow_mut() -= fee as u64;
        **fee_recipient.lamports.borrow_mut() += fee as u64;

        // Create the UserVault
        let user_vault = &mut ctx.accounts.user_vault;
        user_vault.bump = *ctx.bumps.get("user_vault").unwrap();
        user_vault.authority = ctx.accounts.user.key(); 
        user_vault.deposited = 0;
        user_vault.withdrawn = 0;
        user_vault.net_balance = 0;
        user_vault.padding = [0; 60];

        Ok(())
    }
}

// Accounts definition
#[derive(Accounts)]
pub struct CreateVault<'info> {
    /// UserVault account that will be created
    #[account(
        init, 
        payer = user, 
        seeds = [UserVault::PREFIX_SEED, user.key().as_ref()], 
        bump, 
        space = UserVault::SPACE 
    )]
    pub user_vault: Account<'info, UserVault>,

    /// The user creating the Vault
    #[account(mut)]
    pub user: Signer<'info>,

    /// The account that will receive the payment of the fee (0.02 SOL)
    #[account(mut)]
    pub fee_recipient: AccountInfo<'info>,

    ///
    pub system_program: Program<'info, System>,
}

// Definition of the UserVault structure
#[account]
pub struct UserVault {
    pub bump: u8,
    pub authority: Pubkey, // Authority of the Vault (user)
    pub deposited: u64, // Total deposited
    pub withdrawn: u64, // Total withdrawn
    pub net_balance: u64, // Net balance
    pub padding: [u8; 60], // Reserved for future expansions
}

impl UserVault {
    pub const PREFIX_SEED: &'static [u8] = b"user_vault"; // 
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>(); // 
}

// Error Case
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to pay the fee.")]
    InsufficientFunds,
}
