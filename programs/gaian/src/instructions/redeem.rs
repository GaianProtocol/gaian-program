use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, Burn, Mint, Token, TokenAccount},
};

use crate::state::gaian::*;

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"gaian".as_ref(), pt_mint.key().as_ref(), yt_mint.key().as_ref()],
        bump = gaian.bump,
        has_one = pt_mint,
        has_one = yt_mint,
    )]
    pub gaian: Box<Account<'info, Gaian>>,
    #[account(
        mut,
        seeds = [b"gaian_vault".as_ref(), pt_mint.key().as_ref(), yt_mint.key().as_ref()],
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
        mut,
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
        mut,
        associated_token::mint = yt_mint,
        associated_token::authority = signer,
    )]
    pub signer_yt_mint_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Redeem<'info> {
    pub fn redeem(
        &self,
        suffix: String,
        amount: u64,
        pt_amount: u64,
        yt_amount: u64,
    ) -> Result<()> {
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

        **self.sol_vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **self.signer.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}
