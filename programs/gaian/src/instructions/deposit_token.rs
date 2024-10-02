use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer_checked, Mint, MintTo, Token, TokenAccount, TransferChecked},
};

use crate::error::ErrorCode;
use crate::state::gaian::*;

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"gaian"],
        bump = gaian.bump,
        has_one = pt_mint,
        has_one = yt_mint,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = gaian
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    pub pt_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = pt_mint,
        associated_token::authority = signer,
    )]
    pub signer_pt_mint_ata: Account<'info, TokenAccount>,
    pub yt_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = yt_mint,
        associated_token::authority = signer,
    )]
    pub signer_yt_mint_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositToken<'info> {
    fn mint_pt(&self, suffix: &str, amount: u64, bump: u8) -> Result<()> {
        // let seeds = &[b"gaian_pt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        let seeds = &["gaian_pt".as_bytes(), &[bump]];
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
        // let seeds = &[b"gaian_yt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        let seeds = &["gaian_yt".as_bytes(), &[bump]];
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
        if self.gaian.mint.is_none() {
            return Err(ErrorCode::InvalidMint.into());
        }
        if self.gaian.mint.unwrap() != self.mint.key() {
            return Err(ErrorCode::InvalidMint.into());
        }

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.signer_ata.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.signer.to_account_info(),
                },
            ),
            amount,
            self.mint.decimals,
        )?;

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
