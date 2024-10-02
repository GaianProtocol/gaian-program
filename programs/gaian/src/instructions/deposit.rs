use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::gaian::*;

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"gaian"],
        bump = gaian.bump,
        has_one = pt_mint,
        has_one = yt_mint,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    #[account(
        mut,
        seeds = [b"gaian_vault"],
        bump,
    )]
    pub sol_vault: Box<Account<'info, SolVault>>,
    #[account(
        mut,
        seeds = [b"gaian_pt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = pt_mint,
        associated_token::authority = signer,
    )]
    pub signer_pt_mint_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"gaian_yt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
    )]
    pub yt_mint: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = yt_mint,
        associated_token::authority = signer,
    )]
    pub signer_yt_mint_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    fn mint_pt(&self, suffix: &str, amount: u64, bump: u8) -> Result<()> {
        let seeds = &[b"gaian_pt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        // let seeds = &["gaian_pt".as_bytes(), &[bump]];
        let signer = &[&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.pt_mint.to_account_info(),
                    to: self.signer_pt_mint_ata.to_account_info(),
                    authority: self.pt_mint.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        Ok(())
    }

    fn mint_yt(&self, suffix: &str, amount: u64, bump: u8) -> Result<()> {
        let seeds = &[b"gaian_yt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        // let seeds = &["gaian_yt".as_bytes(), &[bump]];
        let signer = &[&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.yt_mint.to_account_info(),
                    to: self.signer_yt_mint_ata.to_account_info(),
                    authority: self.yt_mint.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn deposit(&self, suffix: String, amount: u64, pt_bump: u8, yt_bump: u8) -> Result<()> {
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.signer.to_account_info().clone(),
                to: self.sol_vault.to_account_info().clone(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        self.mint_pt(&suffix, amount, pt_bump)?;
        self.mint_yt(&suffix, amount, yt_bump)?;

        // let pt_seeds = &["gaian_pt".as_bytes(), suffix.as_bytes(), &[pt_bump]];
        // self.mint(pt_seeds, &self.pt_mint, &self.signer_pt_mint_ata, amount)?;
        //
        // let yt_seeds = &["gaian_yt".as_bytes(), suffix.as_bytes(), &[yt_bump]];
        // self.mint(yt_seeds, &self.yt_mint, &self.signer_yt_mint_ata, amount)?;

        Ok(())
    }
}
