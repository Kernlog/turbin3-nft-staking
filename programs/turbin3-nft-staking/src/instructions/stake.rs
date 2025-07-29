use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::{
    error::StakeError,
    state::{StakeConfig, UserAccount, StakeAccount},
};

// Stakes an NFT and begins the reward accumulation process
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    // User's token account holding the NFT to be staked
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    // NFT metadata for collection verification
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    // NFT edition information
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

    // New stake record for this specific NFT
    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {

        // Verify user hasn't exceeded maximum stake limit
        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached);

        // Create stake record with ownership and timestamp
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        // Delegate NFT control to the staking program
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.user_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?;

        // Prepare PDA signing for freeze operation
        let mint_key = self.mint.key();
        let config_key = self.config.key();
        let seeds = &[
            b"stake",
            mint_key.as_ref(),
            config_key.as_ref(),
            &[self.stake_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Freeze the NFT to prevent transfers
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        // Update user's stake count
        self.user_account.amount_staked += 1;

        Ok(())
    } 
}