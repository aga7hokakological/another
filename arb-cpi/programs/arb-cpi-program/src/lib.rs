use anchor_lang::prelude::*;

declare_id!("7nZ7zriZRV7UKWDsaB6kFGKguc5AbdVC4av6vdJnzpqs");

#[program]
pub mod arb_cpi_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        Ok(())
    }

    pub fn set_password(ctx: Context<SetPassword>, pass: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.password = pass;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 32 + 8
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPassword<'info> {
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct User {
    pub password: String,
}