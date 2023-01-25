use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod erc20 {
    use super::*;

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        // Implement mint logic
        // Now it is mint on demand
        ctx.accounts.account.balance += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
            init_if_needed,
            payer = user,
            space = 8 + 8,
            seeds = [b"createAccount", user.key().as_ref()],
            bump,
            )]
    account: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct TokenAccount {
    balance: u64,
}
