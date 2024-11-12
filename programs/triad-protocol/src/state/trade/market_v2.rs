use anchor_lang::prelude::*;

use crate::{ state::OrderDirection, events::PriceUpdate };

#[account]
pub struct MarketV2 {
    pub bump: u8,
    pub authority: Pubkey,
    /// Unique identifier for the market
    pub market_id: u64,
    pub hype_price: u64,
    pub flop_price: u64,
    /// Total liquidity for Hype (in TRD)
    pub hype_liquidity: u64,
    /// Total liquidity for Flop (in TRD)
    pub flop_liquidity: u64,
    /// Total number of Hype shares issued
    pub hype_shares: u64,
    /// Total number of Flop shares issued
    pub flop_shares: u64,
    /// Total trading volume (in TRD)
    pub volume: u64,
    /// Mint $TRD token
    pub mint: Pubkey,
    pub update_ts: i64,
    /// Total number of open orders in this market
    pub opened_orders: u64,
    /// Next available order ID
    pub next_order_id: u64,
    /// Fees applied to trades (in basis points, e.g., 2.131% fee)
    pub fee_bps: u16,
    pub nft_holders_fee_available: u64,
    pub nft_holders_fee_claimed: u64,
    pub market_fee_available: u64,
    pub market_fee_claimed: u64,
    /// Whether the market is currently active for trading
    pub is_active: bool,
    /// Index of the current week in the weekly_results array initialized with default values
    pub market_start: i64,
    pub market_end: i64,
    /// The question or prediction topic for the current week
    pub question: [u8; 80],
    pub winning_direction: WinningDirection,
    pub market_liquidity_at_start: u64,
    pub padding: [u8; 92],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum WinningDirection {
    None,
    Hype,
    Flop,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketArgs {
    pub name: String,
    pub market_id: u64,
    pub question: [u8; 80],
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddLiquidityArgs {
    pub amount: u64,
    pub direction: OrderDirection,
}

impl Default for MarketV2 {
    fn default() -> Self {
        Self {
            bump: 0,
            authority: Pubkey::default(),
            market_id: 0,
            hype_price: 500_000, // Initial price set to 0.5 TRD
            flop_price: 500_000, // Initial price set to 0.5 TRD
            hype_liquidity: 5_000_000_000,
            flop_liquidity: 5_000_000_000,
            mint: Pubkey::default(),
            update_ts: 0,
            next_order_id: 0,
            fee_bps: 2100, // 2.100% fee
            is_active: true,
            market_start: 0,
            market_end: 0,
            hype_shares: 0,
            flop_shares: 0,
            volume: 0,
            opened_orders: 0,
            nft_holders_fee_available: 0,
            nft_holders_fee_claimed: 0,
            market_fee_available: 0,
            market_fee_claimed: 0,
            winning_direction: WinningDirection::None,
            question: [0; 80],
            market_liquidity_at_start: 10_000_000_000,
            padding: [0; 92],
        }
    }
}

impl MarketV2 {
    pub const PREFIX_SEED: &'static [u8] = b"market";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn next_order_id(&mut self) -> u64 {
        let id = self.next_order_id;
        self.next_order_id = self.next_order_id.wrapping_add(1);
        id
    }

    pub fn update_price(
        &mut self,
        amount: u64,
        future_price: u64,
        direction: OrderDirection,
        is_open: bool
    ) -> Result<()> {
        let current_price = match direction {
            OrderDirection::Hype => self.hype_price,
            OrderDirection::Flop => self.flop_price,
        };

        let price_diff = if future_price > current_price {
            future_price - current_price
        } else {
            current_price - future_price
        };

        let price_adjustment = ((price_diff as f64) / 1.02) as u64;

        let new_price = if is_open {
            current_price.checked_add(price_adjustment).unwrap().clamp(1, 999_999)
        } else {
            current_price.checked_sub(price_adjustment).unwrap().clamp(1, 999_999)
        };

        match direction {
            OrderDirection::Hype => {
                self.hype_price = new_price.clamp(1, 999_999);

                if is_open {
                    self.hype_liquidity = self.hype_liquidity.checked_add(amount).unwrap();
                } else {
                    self.hype_liquidity = self.hype_liquidity.checked_sub(amount).unwrap();
                }

                self.flop_price = 1_000_000 - self.hype_price;
            }
            OrderDirection::Flop => {
                self.flop_price = new_price.clamp(1, 999_999);

                if is_open {
                    self.flop_liquidity = self.flop_liquidity.checked_add(amount).unwrap();
                } else {
                    self.flop_liquidity = self.flop_liquidity.checked_sub(amount).unwrap();
                }

                self.hype_price = 1_000_000 - self.flop_price;
            }
        }

        self.hype_price = self.hype_price.clamp(1, 999_999);
        self.flop_price = self.flop_price.clamp(1, 999_999);

        self.update_ts = Clock::get()?.unix_timestamp;

        emit!(PriceUpdate {
            market_id: self.market_id,
            hype_price: self.hype_price,
            flop_price: self.flop_price,
            direction,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
