use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata, MetadataAccount
    }, 
    token::{
        revoke, 
        Mint, 
        Revoke, 
        Token, 
        TokenAccount
    }
};

use crate::{
    error::StakeError,
    state::{StakeConfig, UserAccount, StakeAccount},
};

// Unstakes an NFT and calculates earned rewards
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub mint: Account<'info, Mint>,

    // User's token account that will receive the unstaked NFT
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    // NFT edition information for thawing
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MetadataAccount>, 

    // Global staking configuration
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>, 

    // User's staking activity tracker
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()], 
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    // Stake record to be closed after unstaking
    #[account(
        mut,
        close = user,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {

        // Calculate time elapsed since staking
        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;

        // Verify minimum staking period has been met
        require!(time_elapsed > self.config.freeze_period, StakeError::FreezePeriodNotExpired);

        // Ensure user has staked NFTs
        require!(self.user_account.amount_staked > 0, StakeError::NoStakedTokens);
        
        // Verify only the original staker can unstake
        require!(self.stake_account.owner == self.user.key(), StakeError::NotOriginalStaker);

        // Calculate and award points based on staking duration
        self.user_account.points += (self.config.points_per_stake as u32) * time_elapsed;

        // Thaw the NFT to allow transfers
        let accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.user_mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };
        
        // Prepare PDA signing for thaw operation
        let mint_key = self.mint.key();
        let config_key = self.config.key();
        let seeds = &[
            b"stake",
            mint_key.as_ref(),
            config_key.as_ref(),
            &[self.stake_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];
            
        ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts)
            .invoke_signed(signer_seeds)?;
        
        // Revoke delegation and return control to user
        let account = Revoke{ 
            source: self.user_mint_ata.to_account_info(), 
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), account);
        revoke(ctx)?;
        
        // Update user's stake count
        self.user_account.amount_staked -= 1;

        Ok(())
    }
}