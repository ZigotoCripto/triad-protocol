use anchor_lang::prelude::*;

#[error_code]
pub enum TriadProtocolError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid position")]
    InvalidPosition,

    #[msg("Invalid withdraw amount")]
    InvalidWithdrawAmount,

    #[msg("Stake is locked")]
    StakeLocked,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("No rewards available")]
    NoRewardsAvailable,

    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("No available order slot")]
    NoAvailableOrderSlot,

    #[msg("Market is inactive")]
    MarketInactive,

    #[msg("Order not found")]
    OrderNotFound,

    #[msg("Question period not started")]
    QuestionPeriodNotStarted,

    #[msg("Question period ended")]
    QuestionPeriodEnded,

    #[msg("Stake vault is locked")]
    StakeVaultLocked,

    #[msg("Market still active")]
    MarketStillActive,

    #[msg("Order not open")]
    OrderNotOpen,

    #[msg("Has opened orders")]
    HasOpenedOrders,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Market not resolved")]
    MarketNotResolved,

    #[msg("Market already resolved")]
    MarketAlreadyResolved,

    #[msg("Concurrent transaction")]
    ConcurrentTransaction,

    #[msg("Collection is full")]
    CollectionFull,
}
