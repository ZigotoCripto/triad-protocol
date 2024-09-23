use anchor_lang::prelude::*;

use crate::{ state::{ Order, OrderDirection, OrderStatus, OrderType }, events::PriceUpdate };

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
    /// Total trading volume (in TRD)
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
    /// Fees applied to trades (in basis points, e.g., 300 = 3%) but 2.869 for the protocol; 0.1 NFT Holders; 0.031 Market
    pub fee_bps: u16,
    /// Vault to Receive fees
    pub fee_vault: Pubkey,
    /// Whether the market is currently active for trading
    pub is_active: bool,
    pub is_official: bool,
    pub market_price: u64,
    pub padding: [u8; 224],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeMarketArgs {
    pub name: String,
    pub market_id: u64,
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
            hype_liquidity: 0,
            flop_liquidity: 0,
            total_hype_shares: 0,
            total_flop_shares: 0,
            total_volume: 0,
            mint: Pubkey::default(),
            ts: 0,
            update_ts: 0,
            open_orders_count: 0,
            next_order_id: 0,
            fee_bps: 300, // 3% fee
            fee_vault: Pubkey::default(),
            is_active: true,
            is_official: true,
            market_price: 0,
            padding: [0; 224],
        }
    }
}

impl Market {
    pub const PREFIX_SEED: &'static [u8] = b"market";

    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn next_order_id(&mut self) -> u64 {
        let id = self.next_order_id;
        self.next_order_id = self.next_order_id.wrapping_add(1);
        id
    }

    pub fn calculate_shares(&self, amount: u64, direction: OrderDirection) -> u64 {
        let price = match direction {
            OrderDirection::Hype => self.hype_price,
            OrderDirection::Flop => self.flop_price,
        };

        ((amount * 1_000_000) / price) as u64
    }

    pub fn update_shares(&mut self, shares: u64, direction: OrderDirection) {
        match direction {
            OrderDirection::Hype => {
                self.total_hype_shares = self.total_hype_shares.saturating_add(shares);
            }
            OrderDirection::Flop => {
                self.total_flop_shares = self.total_flop_shares.saturating_add(shares);
            }
        }
    }

    pub fn update_price(
        &mut self,
        amount: u64,
        direction: OrderDirection,
        order_type: OrderType,
        comment: Option<[u8; 64]>
    ) -> Result<()> {
        let price_impact = ((amount as f64) / 1_000_000.0).min(0.1); // Max 10% impact

        match direction {
            OrderDirection::Hype => {
                let new_hype_price = ((self.hype_price as f64) * (1.0 + price_impact)) as u64;
                self.hype_price = new_hype_price.min(1_000_000);
                self.flop_price = 1_000_000 - self.hype_price;
            }
            OrderDirection::Flop => {
                let new_flop_price = ((self.flop_price as f64) * (1.0 + price_impact)) as u64;
                self.flop_price = new_flop_price.min(1_000_000);
                self.hype_price = 1_000_000 - self.flop_price;
            }
        }

        self.hype_price = self.hype_price.max(1);
        self.flop_price = self.flop_price.max(1);

        self.update_ts = Clock::get()?.unix_timestamp;

        let market_price = self.hype_price.max(self.flop_price);
        self.market_price = market_price;

        emit!(PriceUpdate {
            market_id: self.market_id,
            hype_price: self.hype_price,
            flop_price: self.flop_price,
            direction: direction,
            market_price: market_price,
            timestamp: Clock::get()?.unix_timestamp,
            comment: if order_type == OrderType::Market {
                comment
            } else {
                None
            },
        });

        Ok(())
    }

    pub fn get_winning_direction(&self) -> Option<OrderDirection> {
        if self.hype_price > self.flop_price {
            Some(OrderDirection::Hype)
        } else if self.flop_price > self.hype_price {
            Some(OrderDirection::Flop)
        } else {
            None // Prices are equal, no clear winner
        }
    }

    pub fn fill_order_internal<'info>(
        &mut self,
        order: &mut Order,
        amount: u64
    ) -> Result<(u64, u64)> {
        let current_price = match order.direction {
            OrderDirection::Hype => self.hype_price,
            OrderDirection::Flop => self.flop_price,
        };

        let available_liquidity = match order.direction {
            OrderDirection::Hype => self.flop_liquidity,
            OrderDirection::Flop => self.hype_liquidity,
        };

        let fill_amount = amount.min(available_liquidity);
        let fill_shares = ((fill_amount * 1_000_000) / current_price) as u64;

        if fill_amount > 0 {
            order.filled_amount = order.filled_amount.checked_add(fill_amount).unwrap();
            order.filled_shares = order.filled_shares.checked_add(fill_shares).unwrap();

            if order.filled_amount == order.total_amount {
                order.status = OrderStatus::Filled;
            }

            self.update_price(current_price, order.direction, order.order_type, None)?;

            match order.direction {
                OrderDirection::Hype => {
                    self.hype_liquidity = self.hype_liquidity.checked_add(fill_amount).unwrap();
                    self.flop_liquidity = self.flop_liquidity.checked_sub(fill_amount).unwrap();
                }
                OrderDirection::Flop => {
                    self.flop_liquidity = self.flop_liquidity.checked_add(fill_amount).unwrap();
                    self.hype_liquidity = self.hype_liquidity.checked_sub(fill_amount).unwrap();
                }
            }
        }

        Ok((fill_amount, fill_shares))
    }
}
