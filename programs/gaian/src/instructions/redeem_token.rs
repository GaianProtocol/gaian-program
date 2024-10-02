use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer_checked, Burn, Mint, Token, TokenAccount, TransferChecked},
};

use crate::error::ErrorCode;
use crate::state::gaian::*;

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct RedeemToken<'info> {
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

impl<'info> RedeemToken<'info> {
    pub fn redeem(
        &self,
        suffix: String,
        amount: u64,
        pt_amount: u64,
        yt_amount: u64,
    ) -> Result<()> {
        if self.gaian.mint.is_none() {
            return Err(ErrorCode::InvalidMint.into());
        }
        if self.gaian.mint.unwrap() != self.mint.key() {
            return Err(ErrorCode::InvalidMint.into());
        }

        burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    from: self.signer_pt_mint_ata.to_account_info(),
                    mint: self.pt_mint.to_account_info(),
                    authority: self.signer.to_account_info(),
                },
            ),
            pt_amount,
        )?;

        burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    from: self.signer_yt_mint_ata.to_account_info(),
                    mint: self.yt_mint.to_account_info(),
                    authority: self.signer.to_account_info(),
                },
            ),
            yt_amount,
        )?;

        // let seeds = &["gaian".as_bytes(), suffix.as_bytes(), &[self.gaian.bump]];
        let seeds = &["gaian".as_bytes(), &[self.gaian.bump]];
        let signer = &[&seeds[..]];

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.signer_ata.to_account_info(),
                    authority: self.gaian.to_account_info(),
                },
            )
            .with_signer(signer),
            amount,
            self.mint.decimals,
        )?;

        Ok(())
    }
}
