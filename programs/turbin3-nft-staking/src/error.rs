use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("User has reached the maximum number of NFTs they can stake")]
    MaxStakeReached,
    
    #[msg("User has no NFTs currently staked")]
    NoStakedTokens,
    
    #[msg("NFT cannot be unstaked yet - minimum staking period not met")]
    FreezePeriodNotExpired,
    
    #[msg("Only the original staker can unstake this NFT")]
    NotOriginalStaker,
    
    #[msg("The provided stake account is invalid for this NFT")]
    InvalidStakeAccount,
}