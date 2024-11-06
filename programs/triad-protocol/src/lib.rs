use anchor_lang::prelude::*;
use instructions::*;
use state::*;

mod constants;
mod constraints;
mod errors;
mod events;
pub mod instructions;
pub mod state;

declare_id!("TRDwq3BN4mP3m9KsuNUWSN6QDff93VKGSwE95Jbr9Ss");

#[program]
pub mod triad_protocol {
    use super::*;

    pub fn withdraw_v1(ctx: Context<WithdrawV1>, position_index: u8) -> Result<()> {
        instructions::withdraw_v1(ctx, position_index)
    }

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        args: InitializeMarketArgs
    ) -> Result<()> {
        instructions::initialize_market(ctx, args)
    }

    pub fn collect_fee(ctx: Context<CollectFee>) -> Result<()> {
        instructions::collect_fee(ctx)
    }

    pub fn resolve_market(
        ctx: Context<ResolveMarket>,
        winning_direction: WinningDirection
    ) -> Result<()> {
        instructions::resolve_market(ctx, winning_direction)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, args: AddLiquidityArgs) -> Result<()> {
        instructions::add_liquidity(ctx, args)
    }

    pub fn create_user(ctx: Context<CreateUser>, args: CreateUserArgs) -> Result<()> {
        instructions::create_user(ctx, args)
    }

    pub fn create_user_trade(ctx: Context<CreateUserTrade>) -> Result<()> {
        instructions::create_user_trade(ctx)
    }

    pub fn open_order(ctx: Context<OpenOrder>, args: OpenOrderArgs) -> Result<()> {
        instructions::open_order(ctx, args)
    }

    pub fn close_order(ctx: Context<CloseOrder>, order_id: u64) -> Result<()> {
        instructions::close_order(ctx, order_id)
    }

    pub fn settle_order(ctx: Context<SettleOrder>, order_id: u64) -> Result<()> {
        instructions::settle_order(ctx, order_id)
    }

    pub fn stake_nft(ctx: Context<StakeNFT>, args: StakeNFTArgs) -> Result<()> {
        instructions::stake_nft(ctx, args)
    }

    pub fn stake_token(ctx: Context<StakeToken>, args: StakeTokenArgs) -> Result<()> {
        instructions::stake_token(ctx, args)
    }

    pub fn request_withdraw_stake(ctx: Context<RequestWithdrawStake>) -> Result<()> {
        instructions::request_withdraw_stake(ctx)
    }

    pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
        instructions::withdraw_stake(ctx)
    }

    pub fn update_stake_vault(
        ctx: Context<UpdateStakeVault>,
        args: UpdateStakeVaultArgs
    ) -> Result<()> {
        instructions::update_stake_vault(ctx, args)
    }

    pub fn claim_stake_rewards(
        ctx: Context<ClaimStakeRewards>,
        args: ClaimStakeRewardsArgs
    ) -> Result<u64> {
        instructions::claim_stake_rewards(ctx, args)
    }

    pub fn update_stake_boost(ctx: Context<UpdateStakeBoost>) -> Result<()> {
        instructions::update_stake_boost(ctx)
    }
}
