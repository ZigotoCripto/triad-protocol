use anchor_lang::prelude::*;

#[account]
pub struct Collection {
    pub authority: Pubkey,
    pub bump: u8,
    pub symbol: String,
    pub minted: u64,
    pub supply: u64,
    pub padding: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
}

impl Collection {
    pub const PREFIX_SEED: &'static [u8] = b"collection";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}
