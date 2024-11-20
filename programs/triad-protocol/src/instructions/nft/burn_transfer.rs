use anchor_lang::prelude::*;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use mpl_token_metadata::instruction::{burn_nft, transfer_nft};
use anchor_spl::{
    token::{self, Burn, TokenAccount, Transfer},
};


#[program]
pub mod burn_and_transfer_mpl {
    use super::*;

    pub fn burn_and_transfer(ctx: Context<BurnAndTransfer>, rarity: String) -> Result<()> {
        // Burn the TICKET NFT
        let ticket_metadata = Metadata::from_account_info(&ctx.accounts.ticket_metadata)?;
        validate_rarity(&ticket_metadata, &rarity)?;

        let has_badge = check_badge(&ticket_metadata)?;

        mpl_token_metadata::instruction::burn_nft(
            ctx.accounts.token_program.key(),
            ctx.accounts.signer.key(),
            ctx.accounts.user_ticket_ata.key(),
            ctx.accounts.ticket_mint.key(),
            ctx.accounts.ticket_metadata.key(),
        )?;

        // Find a Vault NFT with matching rarity
        let nft_to_transfer = find_nft_in_vault(&ctx.accounts.vault_metadata, &rarity)?;

        // Transfer the Vault NFT to the user
        mpl_token_metadata::instruction::transfer_nft(
            ctx.accounts.token_program.key(),
            ctx.accounts.vault.key(),
            ctx.accounts.vault_nft_ata.key(),
            ctx.accounts.user_nft_ata.key(),
            ctx.accounts.nft_mint.key(),
            ctx.accounts.vault_metadata.key(),
        )?;

        // Log the NFT number for future boosts if the TICKET NFT had a badge
        if has_badge {
            let vault_metadata = Metadata::from_account_info(&ctx.accounts.vault_metadata)?;
            log_nft_number(&mut ctx.accounts.boost_registry, &vault_metadata)?;
        }

        Ok(())
    }
}

// Validates that the rarity of the NFT matches the expected value
fn validate_rarity(metadata: &Metadata, expected_rarity: &str) -> Result<()> {
    let attributes = metadata.data.attributes.ok_or(BurnAndTransferError::MetadataNotFound)?;
    for attribute in attributes {
        if attribute.trait_type == "Rarity" && attribute.value == expected_rarity {
            return Ok(());
        }
    }
    Err(BurnAndTransferError::InvalidRarity.into())
}

// Checks if the TICKET NFT has a specific badge
fn check_badge(metadata: &Metadata) -> Result<bool> {
    let attributes = metadata.data.attributes.ok_or(BurnAndTransferError::MetadataNotFound)?;
    for attribute in attributes {
        if attribute.trait_type == "Badge" && attribute.value {
            return Ok(true);
        }
    }
    Ok(false)
}

// Logs the NFT number for future boosts
fn log_nft_number(boost_registry: &mut Account<BoostRegistry>, metadata: &Metadata) -> Result<()> {
    let nft_number = metadata.data.name.split('#').last() //considering that nft has a #number on the end
        .ok_or(BurnAndTransferError::InvalidMetadataFormat)?
        .parse::<u64>()
        .map_err(|_| BurnAndTransferError::InvalidMetadataFormat)?;

    boost_registry.boosted_nft_numbers.push(nft_number);
    Ok(())
}

// Finds an NFT in the Vault that matches the specified rarity
fn find_nft_in_vault(vault_metadata: &AccountInfo, rarity: &str) -> Result<Pubkey> {
    let metadata = Metadata::from_account_info(vault_metadata)?;
    validate_rarity(&metadata, rarity)?;
    Ok(metadata.mint)
}

// Program context
#[derive(Accounts)]
pub struct BurnAndTransfer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub vault: Signer<'info>,

    #[account(mut)]
    pub ticket_mint: AccountInfo<'info>,

    #[account(mut)]
    pub user_ticket_ata: Account<'info, TokenAccount>,

    /// CHECK: Metadata account for the TICKET NFT (on-chain in mpl_core)
    pub ticket_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,

    /// CHECK: Metadata account for the Vault NFT (on-chain in mpl_core)
    pub vault_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub user_nft_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_nft_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + BoostRegistry::MAX_SIZE
    )]
    pub boost_registry: Account<'info, BoostRegistry>,

    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// Account to store boosted NFT numbers
#[account]
pub struct BoostRegistry {
    pub boosted_nft_numbers: Vec<u64>,
}

impl BoostRegistry {
    pub const MAX_SIZE: usize = 8 + 1000 * 8; // Allow storing up to 1000 NFT numbers
}

// Custom errors for the program
#[error_code]
pub enum BurnAndTransferError {
    #[msg("Metadata not found for this NFT.")]
    MetadataNotFound,
    #[msg("Invalid rarity or rarity does not match.")]
    InvalidRarity,
    #[msg("Invalid metadata format. Could not parse NFT number.")]
    InvalidMetadataFormat,
}
