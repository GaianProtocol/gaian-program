use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token::{Mint, Token},
};
use mpl_token_metadata::{accounts::Metadata as MetadataAccount, types::DataV2};

#[derive(Accounts)]
#[instruction(suffix: String)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = [b"gaian_pt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
        payer = signer,
        mint::decimals = 9,
        mint::authority = pt_mint,
    )]
    pub pt_mint: Box<Account<'info, Mint>>,
    ///CHECK: Using "address" constraint to validate metadata account address
    #[account(
        mut,
        address = MetadataAccount::find_pda(&pt_mint.key()).0,
    )]
    pub pt_metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"gaian_yt".as_ref(), suffix.as_bytes().as_ref()],
        bump,
        payer = signer,
        mint::decimals = 9,
        mint::authority = yt_mint,
    )]
    pub yt_mint: Box<Account<'info, Mint>>,
    ///CHECK: Using "address" constraint to validate metadata account address
    #[account(
        mut,
        address = MetadataAccount::find_pda(&yt_mint.key()).0,
    )]
    pub yt_metadata_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateToken<'info> {
    fn create_pt_token(&self, suffix: &str, bump: u8) -> Result<()> {
        let seeds = &[b"gaian_pt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let data_v2 = DataV2 {
            name: format!("PT {}", suffix),
            symbol: format!("PT-{}", suffix),
            uri: "https://cdn.gaian.network/tokens/metadata/pt.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        // CPI Context
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.pt_metadata_account.to_account_info(), // the metadata account being created
                mint: self.pt_mint.to_account_info(), // the mint account of the metadata account
                mint_authority: self.pt_mint.to_account_info(), // the mint authority of the mint account
                update_authority: self.pt_mint.to_account_info(), // the update authority of the metadata account
                payer: self.signer.to_account_info(), // the payer for creating the metadata account
                system_program: self.system_program.to_account_info(), // the system program account
                rent: self.rent.to_account_info(),    // the rent sysvar account
            },
            signer,
        );

        create_metadata_accounts_v3(
            cpi_ctx, // cpi context
            data_v2, // token metadata
            true,    // is_mutable
            true,    // update_authority_is_signer
            None,    // collection details
        )?;

        Ok(())
    }

    fn create_yt_token(&self, suffix: &str, bump: u8) -> Result<()> {
        let seeds = &[b"gaian_yt".as_ref(), suffix.as_bytes().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let data_v2 = DataV2 {
            name: format!("YT {}", suffix),
            symbol: format!("YT-{}", suffix),
            uri: "https://cdn.gaian.network/tokens/metadata/yt.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        // CPI Context
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.yt_metadata_account.to_account_info(), // the metadata account being created
                mint: self.yt_mint.to_account_info(), // the mint account of the metadata account
                mint_authority: self.yt_mint.to_account_info(), // the mint authority of the mint account
                update_authority: self.yt_mint.to_account_info(), // the update authority of the metadata account
                payer: self.signer.to_account_info(), // the payer for creating the metadata account
                system_program: self.system_program.to_account_info(), // the system program account
                rent: self.rent.to_account_info(),    // the rent sysvar account
            },
            signer,
        );

        create_metadata_accounts_v3(
            cpi_ctx, // cpi context
            data_v2, // token metadata
            true,    // is_mutable
            true,    // update_authority_is_signer
            None,    // collection details
        )?;

        Ok(())
    }

    pub fn create_token(&mut self, suffix: String, bumps: &CreateTokenBumps) -> Result<()> {
        self.create_pt_token(&suffix, bumps.pt_mint)?;
        self.create_yt_token(&suffix, bumps.yt_mint)?;

        Ok(())
    }
}
