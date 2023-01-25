use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod erc20 {
    use super::*;

    pub fn create_account(_ctx: Context<CreateAccount>) -> Result<()> {
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

    pub fn approve(ctx: Context<Approve>, amount: u64) -> Result<()> {
        ctx.accounts.approve_account.approve = amount;
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

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account()]
    account: Account<'info, TokenAccount>,
    /// CHECK
    #[account()]
    operator: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"approveAccount", account.key().as_ref(), operator.key().as_ref()],
        bump,
        )]
    approve_account: Account<'info, ApproveAccount>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferFrom<'info> {
    #[account()]
    user: Signer<'info>,
    #[account()]
    from: Account<'info, TokenAccount>,
    #[account()]
    to: Account<'info, TokenAccount>,

}

#[account]
#[derive(Default)]
pub struct TokenAccount {
    balance: u64,
}

#[account]
#[derive(Default)]
pub struct ApproveAccount {
    approve: u64,
}

#[error_code]
pub enum ERC20Error {
    #[msg("Account has insufficient balance")]
    InsufficientBalance,
}