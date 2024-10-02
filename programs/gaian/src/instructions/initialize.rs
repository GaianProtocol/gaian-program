use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::gaian::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Gaian::INIT_SPACE,
        seeds = [b"gaian"],
        bump,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    #[account(
        init,
        seeds = [b"gaian_vault"],
        bump,
        payer = signer,
        space = SolVault::INIT_SPACE,
    )]
    pub sol_vault: Box<Account<'info, SolVault>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        bumps: &InitializeBumps,
        pt_mint: Pubkey,
        yt_mint: Pubkey,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        self.gaian.set_inner(Gaian {
            bump: bumps.gaian,
            authority: self.signer.key(),
            mint: None,
            pt_mint,
            yt_mint,
            suffix,
            expiration_time,
        });

        Ok(())
    }
}

// --

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Gaian::INIT_SPACE,
        seeds = [b"gaian_token"],
        bump,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = gaian,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeToken<'info> {
    pub fn initialize(
        &mut self,
        bumps: &InitializeTokenBumps,
        pt_mint: Pubkey,
        yt_mint: Pubkey,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        self.gaian.set_inner(Gaian {
            bump: bumps.gaian,
            authority: self.signer.key(),
            mint: Some(self.mint.key()),
            pt_mint,
            yt_mint,
            suffix,
            expiration_time,
        });

        Ok(())
    }
}
