use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::gaian::*;

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Gaian::INIT_SPACE,
        seeds = [b"gaian".as_ref(), pt_mint.key().as_ref(), yt_mint.key().as_ref()],
        bump,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    #[account(
        init,
        seeds = [b"gaian_vault".as_ref(), pt_mint.key().as_ref(), yt_mint.key().as_ref()],
        bump,
        payer = signer,
        space = SolVault::INIT_SPACE,
    )]
    pub sol_vault: Box<Account<'info, SolVault>>,
    #[account(
        seeds = [b"gaian_pt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"gaian_yt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub yt_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        bumps: &InitializeBumps,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        self.gaian.set_inner(Gaian {
            bump: bumps.gaian,
            authority: self.signer.key(),
            mint: None,
            pt_mint: self.pt_mint.key(),
            yt_mint: self.yt_mint.key(),
            expiration_time,
        });

        Ok(())
    }
}

// --

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Gaian::INIT_SPACE,
        seeds = [b"gaian_token", pt_mint.key().as_ref(), yt_mint.key().as_ref()],
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
    #[account(
        seeds = [b"gaian_pt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"gaian_yt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub yt_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeToken<'info> {
    pub fn initialize(
        &mut self,
        bumps: &InitializeTokenBumps,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        self.gaian.set_inner(Gaian {
            bump: bumps.gaian,
            authority: self.signer.key(),
            mint: Some(self.mint.key()),
            pt_mint: self.pt_mint.key(),
            yt_mint: self.yt_mint.key(),
            expiration_time,
        });

        Ok(())
    }
}
