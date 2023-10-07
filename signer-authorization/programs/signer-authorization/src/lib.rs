use anchor_lang::prelude::*;

declare_id!("7GrBo1xfcpmGkS83Lh9M8oZscRzn5EqQyyiWGadDTmEN");

#[program]
pub mod signer_authorization {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0; 
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
    #[account(
        init,
        payer = user,
        space = 8 + 8, 
    )]
    my_account: Account<'info, MyAccount>,
    /// CHECK:
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    my_account: Account<'info, MyAccount>,
    /// CHECK:
    #[account(mut)]
    user: UncheckedAccount<'info>,
}

#[account]
pub struct MyAccount {
    data: u32,
}