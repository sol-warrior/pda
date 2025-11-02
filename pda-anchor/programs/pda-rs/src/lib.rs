use anchor_lang::prelude::*;

declare_id!("8pKd7UzCkLS3og9yk97WSGWehSf4AD7cXLi5Bpj8oJPd");

#[program]
pub mod pda_account {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let account_data = &mut ctx.accounts.pda;
        account_data.user = *ctx.accounts.user.key;

        account_data.bump = ctx.bumps.pda;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds= [b"solwarrior", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + DataAccount::INIT_SPACE

    )]
    pub pda: Account<'info, DataAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,
}
