use anchor_lang::prelude::*;
use store_password::cpi::accounts::SetPassword;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("7nZ7zriZRV7UKWDsaB6kFGKguc5AbdVC4av6vdJnzpqs");

#[program]
pub mod arb_cpi {
use super::*;

pub fn login(ctx: Context<Login>, pass: String) -> Result<()> {
    let context = CpiContext::new(
        ctx.accounts.store_password_program.to_account_info(),
        SetPassword {
            user_account: ctx.accounts.user.to_owned(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    );

    let create_instruction = Instruction {
        program_id: context.program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.user_account.key(), false),
            AccountMeta::new(ctx.accounts.user.key(), true),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ],
        data: pass,
    };

    invoke(
        &create_instruction,
        &context.accounts.to_account_infos(),
    )?;

    Ok(())
}
}

#[derive(Accounts)]
pub struct Login<'info> {
    #[account(mut)]
    pub user: UncheckedAccount<'info>,
    #[account(mut)]
    pub store_password_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}