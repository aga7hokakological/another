use anchor_lang::prelude::*;

declare_id!("CVq7vvAgiCUpCvQjHgoBxcGd2yyzfq8iVPii371SJvhK");

#[program]
pub mod overflow_safe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val1: u8, val2: u8) -> Result<()> {
        let op = &mut ctx.accounts.data;
        op.one = val1;
        op.two = val2;

        Ok(())
    }

    pub fn add(ctx: Context<Addition>) -> Result<()> {
        let op = &mut ctx.accounts.data;

        op.data = op.one.checked_add(op.two).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 2 + 2 + 2 + 8
    )]
    data: Account<'info, Data>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    data: Account<'info, Data>,
    #[account(mut)]
    user: Signer<'info>,
}

#[account]
pub struct Data {
    pub one: u8,
    pub two: u8,
    pub data: u8,
}