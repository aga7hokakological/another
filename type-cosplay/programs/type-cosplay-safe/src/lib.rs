use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Token, Mint, TokenAccount},
};

declare_id!("C1SBUQVojZ8KwHKRCrVLQp37GTJjotL9w3pXFK72pGxA");

#[program]
pub mod type_cosplay_safe {
    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>, key: Pubkey) -> Result<()> {
        let admin = &mut ctx.accounts.admin_account;
        admin.key = key;

        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, key: Pubkey) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        user.key = key;

        Ok(())
    }
    
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        vault.token = ctx.accounts.token_mint.key();
        vault.amount = 0;
        vault.fees = 0;
    
        Ok(())
    }

    pub fn update_vault_fees(ctx: Context<UpdateFees>, fee: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault_account;
        vault.fees = fee;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32
    )]
    pub admin_account: Account<'info, Admin>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = admin,
        space = 32 + 8 + 8 + 8,
    )]
    pub vault_account: Account<'info, Vault>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFees<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub token: Pubkey,
    pub amount: u64,
    pub fees: u64,
}

#[account]
pub struct Admin {
    key: Pubkey,
}

#[account]
pub struct User {
    key: Pubkey,
}