use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("User has insufficient LP token balance.")]
    InsufficientLpBalance,
    #[msg("Only admin can call this function.")]
    Unauthorized,
    #[msg("Overflow or math error.")]
    MathError,
    #[msg("Rewards have ended.")]
    RewardsEnded,
    #[msg("Invalid token mint provided.")]
    InvalidTokenMint,
    #[msg("Invalid owner.")]
    InvalidOwner,
    #[msg("Rewards have not started yet.")]
    RewardsNotStarted,
    #[msg("No LP tokens found.")]
    NoLPTokens,
    #[msg("Insufficient reward balance.")]
    InsufficientRewardBalance,
}
