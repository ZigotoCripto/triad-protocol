use anchor_lang::prelude::*;

use crate::state::WinningDirection;

#[account]
pub struct Market {
    pub bump: u8,
    pub authority: Pubkey,
    /// Unique identifier for the market
    pub market_id: u64,
    /// The event being predicted (e.g., "tJUP/TRD")
    pub name: String,
    /// Current price for Hype outcome (0-1000000, representing 0 to 1 TRD)
    /// 1000000 = 1 TRD, 500000 = 0.5 TRD, etc.
    pub hype_price: u64,
    /// Current price for Flop outcome (0-1000000, representing 0 to 1 TRD)
    pub flop_price: u64,
    /// Total liquidity for Hype (in TRD)
    pub hype_liquidity: u64,
    /// Total liquidity for Flop (in TRD)
    pub flop_liquidity: u64,
    /// Total number of Hype shares issued
    pub total_hype_shares: u64,
    /// Total number of Flop shares issued
    pub total_flop_shares: u64,
    /// Total trading volume (in TRD) for all resolutions
    pub total_volume: u64,
    /// Mint $TRD token
    pub mint: Pubkey,
    /// Timestamp of the init
    pub ts: i64,
    pub update_ts: i64,
    /// Total number of open orders in this market
    pub open_orders_count: u64,
    /// Next available order ID
    pub next_order_id: u64,
    /// Fees applied to trades (in basis points, e.g., 2.131% fee)
    pub fee_bps: u16,
    /// Vault to Receive fees
    pub fee_vault: Pubkey,
    /// Whether the market is currently active for trading
    pub is_active: bool,
    pub market_price: u64,
    pub previous_resolved_question: ResolvedQuestion,
    /// Index of the current week in the weekly_results array initialized with default values
    pub current_question_id: u64,
    pub current_question_start: i64,
    pub current_question_end: i64,
    /// The question or prediction topic for the current week
    pub current_question: [u8; 80],
    pub liquidity: u64,
    pub padding: [u8; 200],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct ResolvedQuestion {
    pub question_id: u64,
    /// The question or prediction topic for this week
    pub question: [u8; 80],
    /// Start timestamp of the week
    pub start_time: i64,
    /// End timestamp of the week
    pub end_time: i64,
    /// Total liquidity for Hype (in TRD)
    pub hype_liquidity: u64,
    /// Total liquidity for Flop (in TRD)
    pub flop_liquidity: u64,
    /// The winning direction (Hype, Flop or None)
    pub winning_direction: WinningDirection,
    pub market_price: u64,
    /// Final price for Hype outcome at the end of the week
    pub final_hype_price: u64,
    /// Final price for Flop outcome at the end of the week
    pub final_flop_price: u64,
    /// Total number of Hype shares issued
    pub total_hype_shares: u64,
    /// Total number of Flop shares issued
    pub total_flop_shares: u64,
    pub padding: [u8; 40],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum QuestionStatus {
    Resolved,
    Unresolved,
}

impl Default for ResolvedQuestion {
    fn default() -> Self {
        Self {
            question_id: 0,
            question: [0; 80],
            start_time: 0,
            end_time: 0,
            hype_liquidity: 0,
            flop_liquidity: 0,
            winning_direction: WinningDirection::None,
            market_price: 0,
            final_hype_price: 500_000,
            final_flop_price: 500_000,
            total_hype_shares: 0,
            total_flop_shares: 0,
            padding: [0; 40],
        }
    }
}

impl Default for Market {
    fn default() -> Self {
        Self {
            bump: 0,
            authority: Pubkey::default(),
            market_id: 0,
            name: String::new(),
            hype_price: 500_000, // Initial price set to 0.5 TRD
            flop_price: 500_000, // Initial price set to 0.5 TRD
            hype_liquidity: 500_000_000,
            flop_liquidity: 500_000_000,
            total_hype_shares: 0,
            total_flop_shares: 0,
            total_volume: 0,
            mint: Pubkey::default(),
            ts: 0,
            update_ts: 0,
            open_orders_count: 0,
            next_order_id: 0,
            fee_bps: 2100, // 2.100% fee
            fee_vault: Pubkey::default(),
            is_active: true,
            market_price: 0,
            previous_resolved_question: ResolvedQuestion::default(),
            current_question_id: 0,
            current_question_start: 0,
            current_question_end: 0,
            current_question: [0; 80],
            liquidity: 0,
            padding: [0; 200],
        }
    }
}

impl Market {
    pub const PREFIX_SEED: &'static [u8] = b"market";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();
}
