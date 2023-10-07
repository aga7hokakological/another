use anchor_lang::prelude::*;

declare_id!("HFeMKELQjfX6vCM79KNWbAQWVyviRqjQQn56dN7AmsSE");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
