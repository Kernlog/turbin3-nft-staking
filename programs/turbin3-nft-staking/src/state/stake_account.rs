use anchor_lang::prelude::*;

// Records individual NFT staking instances
#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey, // Address of the user who staked the NFT
    pub mint: Pubkey, // Unique identifier of the staked NFT
    pub staked_at: i64, // Unix timestamp when the NFT was staked
    pub bump: u8, 
    // Note: Per-NFT reward tracking could be added here if needed
    // pub accumulated_rewards: u64,
    // pub last_claimed_rewards: u64,
}