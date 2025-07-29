use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::state::{StakeConfig, UserAccount};

// Converts accumulated reward points into actual tokens
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    // User's reward tracking account - will be modified to reset points
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    
    // Reward token mint - creates new tokens when users claim
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,
    
    // Global configuration - acts as mint authority for rewards
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    
    // User's reward token account - will receive the minted tokens
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        // Prepare cross-program invocation to token program
        let cpi_program = self.token_program.to_account_info();

        // Create signer seeds for PDA authority
        let seeds = &[
            b"config".as_ref(),      
            &[self.config.bump]      
        ];
        let signer_seeds = &[&seeds[..]];

        // Define the mint operation parameters
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        // Execute the mint operation with proper decimal conversion
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Convert points to token atomic units based on mint decimals
        mint_to(
            cpi_context,
            self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32)
        )?;

        // Reset user points after successful claim
        self.user_account.points = 0;
        
        Ok(())
    }
}