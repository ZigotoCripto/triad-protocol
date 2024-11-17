use anchor_lang::prelude::*;
use std::fmt;

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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTicketArgs {
    pub collection_symbol: String,
    pub discount: u64,
    pub is_boosted: bool,
    pub number: u64,
    pub rarity: Rarity,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rarity::Common => write!(f, "Common"),
            Rarity::Uncommon => write!(f, "Uncommon"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::Epic => write!(f, "Epic"),
            Rarity::Legendary => write!(f, "Legendary"),
            Rarity::Mythic => write!(f, "Mythic"),
        }
    }
}

#[account]
pub struct Nft {}

impl Collection {
    pub const PREFIX_SEED: &'static [u8] = b"collection";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}
