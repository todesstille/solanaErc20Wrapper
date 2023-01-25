use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod erc20 {
    use super::*;

    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        Ok(())
    }


    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        // Implement mint logic
        // Now it is mint on demand
        ctx.accounts.account.balance += amount;
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        require!(ctx.accounts.account1.balance >= amount, ERC20Error::InsufficientBalance);
        ctx.accounts.account1.balance -= amount;
        ctx.accounts.account2.balance += amount;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
            init,
            payer = user,
            space = 8 + 8,
            seeds = [b"createAccount", user.key().as_ref()],
            bump,
            )]
    account: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    account1: Account<'info, TokenAccount>,
    #[account(mut)]
    account2: Account<'info, TokenAccount>,
}

#[account]
#[derive(Default)]
pub struct TokenAccount {
    balance: u64,
}

#[error_code]
pub enum ERC20Error {
    #[msg("Account has insufficient balance")]
    InsufficientBalance,
}