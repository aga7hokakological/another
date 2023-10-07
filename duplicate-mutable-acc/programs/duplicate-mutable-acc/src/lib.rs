use anchor_lang::prelude::*;

declare_id!("E8jXzmuzzTe823b4gYpdayUZRTaviJWMiWJ5WmNKDDHk");

#[program]
pub mod duplicate_mutable_acc {
    use super::*;

    pub fn initialize_accounts(ctx: Context<InitializeAccounts>) -> Result<()> {
        let account1 = &mut ctx.accounts.account_one;
        account1.data = 0;
        
        let account2 = &mut ctx.accounts.account_two;
        account2.data = 0;

        Ok(())
    }

    pub fn who_won(ctx: Context<UpdateAccount>, number1: u64, number2: u64) -> Result<()> {
        let account1 = &mut ctx.accounts.account_one;
        account1.data = number1;
        
        let account2 = &mut ctx.accounts.account_two;
        account2.data = number2;

        if account1.data > account2.data {
            msg!("user 1 won");
            account1.amount += 100;
        } else {
            msg!("user 2 won");
            account2.amount += 100;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 8
    )]
    pub account_one: Account<'info, AccData>,
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 8
    )]
    pub account_two: Account<'info, AccData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccount<'info> {
    #[account(mut)]
    pub account_one: Account<'info, AccData>,
    #[account(mut)]
    pub account_two: Account<'info, AccData>,
    // #[account(mut)]
    // pub signer: Signer<'info>,
}

#[account]
pub struct AccData {
    pub data: u64,
    pub amount: u64,
}