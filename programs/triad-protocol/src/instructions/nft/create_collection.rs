use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use mpl_core::{
    ID as CORE_PROGRAM_ID,
    instructions::CreateCollectionV2CpiBuilder,
    types::{ PluginAuthorityPair, Creator },
};

use crate::{ state::Collection, CreateCollectionArgs };

#[derive(Accounts)]
#[instruction(args: CreateCollectionArgs)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub core_collection: Signer<'info>,

    #[account(
        init,
        seeds = [Collection::PREFIX_SEED, args.symbol.as_bytes()],
        bump,
        payer = signer,
        space = Collection::SPACE
    )]
    pub collection: Box<Account<'info, Collection>>,

    /// CHECK: this will be checked by core
    #[account(address = CORE_PROGRAM_ID)]
    pub metaplex_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
    let collection = &mut ctx.accounts.collection;

    collection.set_inner(Collection {
        authority: *ctx.accounts.signer.to_account_info().key,
        bump: ctx.bumps.collection,
        symbol: args.symbol.clone(),
        minted: 0,
        supply: args.supply,
        padding: [0; 64],
    });

    let seeds: &[&[&[u8]]] = &[
        &[Collection::PREFIX_SEED, collection.symbol.as_bytes(), &[ctx.bumps.collection]],
    ];

    let nft_uri = format!(
        "https://shdw-drive.genesysgo.net/9ZgbDbP9wL1oPegdNj66TH6tnazEMFcMnREJdKsKEMwx/{}.json",
        args.symbol
    );

    CreateCollectionV2CpiBuilder::new(&ctx.accounts.metaplex_program.to_account_info())
        .collection(&ctx.accounts.core_collection.to_account_info())
        .update_authority(Some(&collection.to_account_info()))
        .payer(&ctx.accounts.signer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(args.name)
        .uri(nft_uri)
        .plugins(
            vec![PluginAuthorityPair {
                authority: None,
                plugin: mpl_core::types::Plugin::Royalties(mpl_core::types::Royalties {
                    basis_points: 500,
                    creators: vec![Creator {
                        address: *collection.to_account_info().key,
                        percentage: 100,
                    }],
                    rule_set: mpl_core::types::RuleSet::None,
                }),
            }]
        )
        .invoke_signed(seeds)?;

    Ok(())
}
