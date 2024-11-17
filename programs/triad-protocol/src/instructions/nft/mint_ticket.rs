use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV2CpiBuilder;
use anchor_spl::token_2022::spl_token_2022::extension::BaseStateWithExtensions;
use spl_token_metadata_interface::state::TokenMetadata;
use mpl_core::types::DataState::{ self };
use mpl_core::types::{ Creator, PluginAuthorityPair };
use mpl_core::{ ID as CORE_PROGRAM_ID, accounts::BaseCollectionV1 };
use anchor_spl::token_2022::{
    spl_token_2022::{ extension::PodStateWithExtensions, pod::PodMint },
    Token2022,
};
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{ Mint, TokenAccount } };
use std::str::FromStr;
use anchor_spl::token_2022::{ burn, close_account, Burn, CloseAccount };

use crate::{
    errors::TriadProtocolError,
    state::{ Collection, Rarity, Nft },
    MintTicketArgs,
    constraints::is_verifier,
};

#[derive(Accounts)]
#[instruction(args: MintTicketArgs)]
pub struct MintTicket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [Collection::NFT_PREFIX_SEED, args.number.to_le_bytes().as_ref()],
        bump,
        payer = signer,
        space = 8
    )]
    pub nft: Account<'info, Nft>,

    #[account(mut, constraint = is_verifier(&verifier)?)]
    pub verifier: Signer<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(
        mut,
        extensions::metadata_pointer::metadata_address = nft_mint
    )]
    pub nft_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, seeds = [Collection::PREFIX_SEED, args.collection_symbol.as_bytes()], bump)]
    pub collection: Box<Account<'info, Collection>>,

    #[account(mut, constraint = core_collection.update_authority == collection.key())]
    pub core_collection: Account<'info, BaseCollectionV1>,

    #[account(mut)]
    pub trd_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
       mut,
       associated_token::mint = trd_mint,
       associated_token::authority = signer,
       associated_token::token_program = token_program
    )]
    pub user_trd_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: this will be checked by core
    pub metaplex_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_ticket(ctx: Context<MintTicket>, args: MintTicketArgs) -> Result<()> {
    let collection = &mut ctx.accounts.collection;
    let core_collection = &mut ctx.accounts.core_collection;
    let nft_mint = &ctx.accounts.nft_mint.to_account_info();
    let buffer = nft_mint.try_borrow_data()?;
    let state = PodStateWithExtensions::<PodMint>::unpack(&buffer)?;
    let token_metadata = state.get_variable_len_extension::<TokenMetadata>()?;

    if
        token_metadata.update_authority.0 !=
            Pubkey::from_str("4jfd5BshR8oK6ezdFxKYia4g2Z9dJK6tZnpLfKSw1C3C").unwrap() ||
        args.number > collection.supply
    {
        return Err(TriadProtocolError::Unauthorized.into());
    }

    let seeds: &[&[&[u8]]] = &[
        &[b"collection", args.collection_symbol.as_bytes(), &[ctx.bumps.collection]],
    ];

    require!(collection.supply > collection.minted, TriadProtocolError::CollectionFull);

    // Burn the NFT
    burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Burn {
            mint: ctx.accounts.nft_mint.to_account_info(),
            from: ctx.accounts.user_nft_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }),
        1
    )?;
    close_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), CloseAccount {
            account: ctx.accounts.user_nft_ata.to_account_info(),
            destination: ctx.accounts.signer.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        })
    )?;

    // Burn the TRD
    let price = match args.rarity {
        Rarity::Common => 150_000_000,
        Rarity::Uncommon => 100_000_000,
        Rarity::Rare => 50_000_000,
        _ => 0u64,
    };

    let discount = price.checked_mul(args.discount.checked_div(100).unwrap()).unwrap();
    let price_with_discount = price.checked_sub(discount).unwrap();

    burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Burn {
            mint: ctx.accounts.trd_mint.to_account_info(),
            from: ctx.accounts.user_trd_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        }),
        price_with_discount
    )?;

    let number = args.number;
    let nft_name = format!("Poseidon's Ticket #{}", number);

    let rarity_str = args.rarity.to_string();
    let rarity_boost_str = if args.is_boosted { "boosted" } else { "normal" };

    let nft_uri = format!(
        "https://shdw-drive.genesysgo.net/9ZgbDbP9wL1oPegdNj66TH6tnazEMFcMnREJdKsKEMwx/{}_{}.json",
        rarity_str,
        rarity_boost_str
    );

    CreateV2CpiBuilder::new(&ctx.accounts.metaplex_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(Some(&core_collection.to_account_info()))
        .authority(Some(&collection.to_account_info()))
        .payer(&ctx.accounts.signer.to_account_info())
        .update_authority(None)
        .owner(Some(&ctx.accounts.signer.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .data_state(DataState::AccountState)
        .name(nft_name.clone())
        .uri(nft_uri.clone())
        .plugins(
            vec![
                PluginAuthorityPair {
                    authority: None,
                    plugin: mpl_core::types::Plugin::Royalties(mpl_core::types::Royalties {
                        basis_points: 500,
                        creators: vec![Creator {
                            address: collection.key(),
                            percentage: 100,
                        }],
                        rule_set: mpl_core::types::RuleSet::None,
                    }),
                },
                PluginAuthorityPair {
                    authority: None,
                    plugin: mpl_core::types::Plugin::Attributes(mpl_core::types::Attributes {
                        attribute_list: vec![
                            mpl_core::types::Attribute {
                                key: "Boost".to_string(),
                                value: args.is_boosted.to_string(),
                            },
                            mpl_core::types::Attribute {
                                key: "Rarity".to_string(),
                                value: args.rarity.to_string(),
                            }
                        ],
                    }),
                },
                PluginAuthorityPair {
                    authority: None,
                    plugin: mpl_core::types::Plugin::MasterEdition(mpl_core::types::MasterEdition {
                        max_supply: Some(0),
                        name: Some(nft_name.clone()),
                        uri: Some(nft_uri.clone()),
                    }),
                }
            ]
        )
        .invoke_signed(seeds)?;

    collection.minted = collection.minted.checked_add(1).unwrap();

    Ok(())
}
