use anchor_lang::prelude::*;

declare_id!("E627JnBgtEZmTPm3uaToW1nfTKmW4x9svAknrWfTZEVs");

#[program]
pub mod re_initialization {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut my_account = MyAccount::try_from_slice(&ctx.accounts.my_account.data.borrow()).unwrap();
        my_account.data = 0; 
        my_account.serialize(&mut *ctx.accounts.my_account.data.borrow_mut())?;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u32) -> Result<()> {
        let mut my_account = MyAccount::try_from_slice(&ctx.accounts.my_account.data.borrow()).unwrap();
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    my_account: UncheckedAccount<'info>,
    user: Signer<'info>,
    // system_program: SystemProgram<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    /// CHECK:
    #[account(mut)]
    my_account: UncheckedAccount<'info>,
    #[account(mut)]
    user: Signer<'info>,
}

#[account]
pub struct MyAccount {
    data: u32,
}