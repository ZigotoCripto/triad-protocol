use anchor_lang::prelude::*;

use crate::{ errors::TriadProtocolError, state::trade::user_trade::OrderDirection };

#[account]
pub struct Market {
    pub bump: u8,
    pub authority: Pubkey,
    /// Unique identifier for the market
    pub market_id: u64,
    /// The event being predicted (e.g., "tJUP/TRD")
    pub ticker_name: String,
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
    /// Vault token account of $TRD
    pub vault_token_account: Pubkey,
    /// Mint $TRD token
    pub mint: Pubkey,
    /// Timestamp of the last update
    pub last_update_ts: i64,
    /// Total number of open orders in this market
    pub open_orders_count: u64,
    /// Next available order ID
    pub next_order_id: u64,
    /// Fees applied to trades (in basis points, e.g., 300 = 3%)
    pub fee_bps: u16,
    /// Authority to receive fees
    pub fee_authority: Pubkey,
    /// Token account of the fee token
    pub fee_ata: Pubkey,
    /// Whether the market is currently active for trading
    pub is_active: bool,
    pub vault: Pubkey,
    pub is_official: bool,
    pub padding: [u8; 240],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMarketArgs {
    pub name: String,
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

    pub fn update_price(&mut self, new_price: u64, direction: OrderDirection) -> Result<()> {
        if new_price > 1_000_000 {
            return Err(TriadProtocolError::InvalidPrice.into());
        }

        match direction {
            OrderDirection::Hype => {
                self.hype_price = new_price;
            }
            OrderDirection::Flop => {
                self.flop_price = new_price;
            }
        }

        self.last_update_ts = Clock::get()?.unix_timestamp;

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
}
