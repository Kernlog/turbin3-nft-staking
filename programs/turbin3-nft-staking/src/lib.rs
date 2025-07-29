use anchor_lang::prelude::*;

declare_id!("AhXKfFfCbR7GR2P8Wft2DRZNc2gqeSYhUFLP7VETTMYE");

pub mod state;
pub mod instructions;
pub mod error;

pub use instructions::*;

#[program]
pub mod nft_staking {
    use super::*;

    // Sets up the global staking configuration and reward token mint
    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    // Creates a user account to track staking activity and rewards
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    // Allows users to stake their NFTs and start earning rewards
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    // Enables users to unstake their NFTs and calculate earned rewards
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    // Converts accumulated reward points into actual tokens
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}