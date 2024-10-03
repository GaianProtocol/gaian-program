use anchor_lang::prelude::*;

mod instructions;
use instructions::*;
mod error;
mod state;

declare_id!("EZ6wxSVyZgHGSNhg63hh8X9sovbugPMHJ5pG6ntyWQ1x");

#[program]
pub mod gaian {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, suffix: String) -> Result<()> {
        ctx.accounts.create_token(suffix, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize(&ctx.bumps, suffix, expiration_time)?;
        Ok(())
    }

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        suffix: String,
        expiration_time: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize(&ctx.bumps, suffix, expiration_time)?;
        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        suffix: String,
        amount: u64,
        pt_bump: u8,
        yt_pump: u8,
    ) -> Result<()> {
        ctx.accounts.deposit(suffix, amount, pt_bump, yt_pump)?;
        Ok(())
    }

    pub fn deposit_token(
        ctx: Context<DepositToken>,
        suffix: String,
        amount: u64,
        pt_bump: u8,
        yt_pump: u8,
    ) -> Result<()> {
        ctx.accounts.deposit(suffix, amount, pt_bump, yt_pump)?;
        Ok(())
    }

    pub fn redeem(
        ctx: Context<Redeem>,
        suffix: String,
        amount: u64,
        pt_amount: u64,
        yt_amount: u64,
    ) -> Result<()> {
        ctx.accounts.redeem(suffix, amount, pt_amount, yt_amount)?;
        Ok(())
    }

    pub fn redeem_token(
        ctx: Context<RedeemToken>,
        suffix: String,
        amount: u64,
        pt_amount: u64,
        yt_amount: u64,
    ) -> Result<()> {
        ctx.accounts.redeem(suffix, amount, pt_amount, yt_amount)?;
        Ok(())
    }
}
