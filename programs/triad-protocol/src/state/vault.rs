use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    /// The bump for the vault pda
    pub bump: u8,
    /// authority for the vault
    pub authority: Pubkey,
    /// name of the vault
    pub name: String,
    /// token account for the vault e.g. tDRIFT
    pub token_account: Pubkey,
    /// ticker address
    pub ticker_address: Pubkey,
    /// lifetime total deposits
    pub total_deposits: u64,
    /// lifetime total withdraws
    pub total_withdraws: u64,
    /// timestamp vault initialized
    pub init_ts: i64,
    /// lifetime net deposits
    pub net_deposits: i64,
    /// lifetime net withdraws
    pub net_withdraws: i64,
    /// Long bet balance
    pub long_balance: u64,
    /// Short bet balance
    pub short_balance: u64,
    /// Ticker PDA
    pub ticker: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositVaultArgs {
    pub amount: u64,
    pub is_long: bool,
}

impl Vault {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &'static [u8] = b"vault";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>();

    pub fn get_vault_signer_seeds<'a>(name: &'a [u8], bump: &'a u8) -> [&'a [u8]; 3] {
        [b"vault".as_ref(), name, bytemuck::bytes_of(bump)]
    }

    pub const PREFIX_SEED_VAULT_TOKEN_ACCOUNT: &'static [u8] = b"vault_token_account";
}
