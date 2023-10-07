use anchor_lang::prelude::*;

declare_id!("HFeMKELQjfX6vCM79KNWbAQWVyviRqjQQn56dN7AmsSE");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u32) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data; 
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u32) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // #[account(constraint = user.key == &my_account.owner)]
    #[account(
        init,
        payer = user,
        space = 8 + 8, 
    )]
    my_account: Account<'info, MyAccount>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: SystemProgram<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    my_account: Account<'info, MyAccount>,
    #[account(mut)]
    user: Signer<'info>,
}

#[account]
pub struct MyAccount {
    data: u32,
}