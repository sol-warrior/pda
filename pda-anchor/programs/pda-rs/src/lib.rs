use anchor_lang::prelude::*;

declare_id!("8pKd7UzCkLS3og9yk97WSGWehSf4AD7cXLi5Bpj8oJPd");

#[program]
pub mod pda_rs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
