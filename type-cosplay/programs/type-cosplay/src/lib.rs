use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Token, Mint, TokenAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::borrow::{Borrow, BorrowMut};

declare_id!("C1SBUQVojZ8KwHKRCrVLQp37GTJjotL9w3pXFK72pGxA");

#[program]
pub mod type_cosplay {
    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>, key: Pubkey) -> Result<()> {
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.admin.key(),
            &ctx.accounts.admin_account.key(),
            lamports,
            space,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.admin_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let mut account =
            Admin::try_from_slice(&ctx.accounts.admin_account.key.borrow_mut()).unwrap();

        account.key = ctx.accounts.admin.key();
        account.serialize(&mut *ctx.accounts.admin_account.key.borrow_mut())?;

        // msg!("Admin: {}", account.admin.to_string());

        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, key: Pubkey) -> Result<()> {
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.user.key(),
            &ctx.accounts.user_account.key(),
            lamports,
            space,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let mut account = User::try_from_slice(&ctx.accounts.user_account.key.borrow_mut()).unwrap();

        account.key = ctx.accounts.user.key();
        account.serialize(&mut *ctx.accounts.user_account.key.borrow_mut())?;

        // msg!("User: {}", account.user.to_string());

        Ok(())
    }
    
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.admin.key(),
            &ctx.accounts.vault_account.key(),
            lamports,
            space,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.vault_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let mut account = Vault::try_from_slice(&ctx.accounts.vault_account.data.borrow()).unwrap();

        account.token = ctx.accounts.token.key();
        account.amount = 0;
        account.fees = 0;
        account.serialize(&mut *ctx.accounts.vault_account.data.borrow_mut())?;

        // msg!("Vault: {}", account.adf.to_string());
    
        Ok(())
    }

    pub fn update_vault_fees(ctx: Context<UpdateFees>, fee: u64) -> Result<()> {
        let mut account = Vault::try_from_slice(&ctx.accounts.vault_account.data.borrow()).unwrap();
        account.fees = fee;

        account.serialize(&mut *ctx.accounts.vault_account.data.borrow_mut())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(mut)]
    pub admin_account: Signer<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user_account: Signer<'info>,
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Admin {
    key: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    key: Pubkey,
}