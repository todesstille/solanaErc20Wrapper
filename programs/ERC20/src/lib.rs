use anchor_lang::prelude::*;
use anchor_lang::context::CpiContext;
use anchor_spl::{token, token::{Mint, Token, TokenAccount, Transfer, SetAuthority}};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod erc20 {
    use super::*;

    pub fn init_erc20(ctx: Context<InitERC20>, name: String, symbol: String, decimals: u8) -> Result<()> {
        ctx.accounts.info.name = name;
        ctx.accounts.info.symbol = symbol;
        ctx.accounts.info.decimals = decimals;
        ctx.accounts.info.wrap_mint = ctx.accounts.mint_account.key();
        ctx.accounts.info.vault = ctx.accounts.vault.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        
        require!(ctx.accounts.info.wrap_mint == ctx.accounts.token_account.mint, ERC20Error::WrongWrappedToken);
        let (account, _) = Pubkey::find_program_address
        (&[b"wrappedAccount"], ctx.program_id);
        require!(account == ctx.accounts.vault.key(), ERC20Error::WrongWrappedToken);
        ctx.accounts.user_account.balance += amount;
        token::transfer(
            ctx.accounts.transfer_ctx(),
            amount,
        )?;
        Ok(())
    }

    pub fn create_account(_ctx: Context<CreateAccount>) -> Result<()> {
        Ok(())
    }

    pub fn mint(ctx: Context<ERC20Mint>, amount: u64) -> Result<()> {
        // For tests only
        // Should be removed
        ctx.accounts.account.balance += amount;
        Ok(())
    }

    pub fn transfer(ctx: Context<TransferERC20>, amount: u64) -> Result<()> {
        require!(ctx.accounts.check_authority(ctx.program_id), ERC20Error::WrongAuthority);
        require!(ctx.accounts.account1.balance >= amount, ERC20Error::InsufficientBalance);
        ctx.accounts.account1.balance -= amount;
        ctx.accounts.account2.balance += amount;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, _account: Pubkey, _operator: Pubkey, amount: u64) -> Result<()> {
        let (account, _) = Pubkey::find_program_address
                (&[b"createAccount", ctx.accounts.user.key().as_ref()], ctx.program_id);
        require!(account == _account, ERC20Error::WrongAuthority);

        ctx.accounts.approve_account.approve = amount;
        Ok(())
    }

    pub fn transfer_from(ctx: Context<TransferFrom>, amount: u64) -> Result<()> {
        let (account, _) = Pubkey::find_program_address
                (&[b"approveAccount", ctx.accounts.from.key().as_ref(), ctx.accounts.user.key().as_ref()], ctx.program_id);
        require!(account == ctx.accounts.approve_account.key(), ERC20Error::WrongAuthority);
        require!(amount <= ctx.accounts.approve_account.approve, ERC20Error::InsufficientApprove);
        require!(amount <= ctx.accounts.from.balance, ERC20Error::InsufficientBalance);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitERC20<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
            init,
            space = 8 + 4 + 16 + 4 + 8 + 1 + 32 + 32,
            seeds = [b"accountInfo"],
            bump,
            payer = user,
            )]
    info: Account<'info, ERC20Info>,
    mint_account: Account<'info, Mint>,
    #[account(
            init,
            payer = user,
            seeds = [b"wrappedAccount"],
            bump,
            token::mint = mint_account,
            token::authority = info,
            )]
    vault: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account()]
    info: Account<'info, ERC20Info>,
    #[account(mut)]
    token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    vault: Account<'info, TokenAccount>,
    #[account(mut)]
    user_account: Account<'info, ERC20Account>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
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
    account: Account<'info, ERC20Account>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ERC20Mint<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    account: Account<'info, ERC20Account>,
}

#[derive(Accounts)]
pub struct TransferERC20<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    account1: Account<'info, ERC20Account>,
    #[account(mut)]
    account2: Account<'info, ERC20Account>,
}

#[derive(Accounts)]
#[instruction(_account: Pubkey, _operator: Pubkey)]
pub struct Approve<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"approveAccount", _account.as_ref(), _operator.as_ref()],
        bump,
        )]
    approve_account: Account<'info, ApproveAccount>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferFrom<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    from: Account<'info, ERC20Account>,
    #[account(mut)]
    to: Account<'info, ERC20Account>,
    #[account(mut)]
    approve_account: Account<'info, ApproveAccount>,
}

#[account]
#[derive(Default)]
pub struct ERC20Info {
    name: String,
    symbol: String,
    decimals: u8,
    wrap_mint: Pubkey,
    vault: Pubkey,
}

#[account]
#[derive(Default)]
pub struct ERC20Account {
    balance: u64,
}

#[account]
#[derive(Default)]
pub struct ApproveAccount {
    approve: u64,
}

impl<'info> TransferERC20<'info> {
    fn check_authority(&self, id: &Pubkey) -> bool {
        let (account_authority, _bump) =
            Pubkey::find_program_address(&[b"createAccount", self.user.key().as_ref()], id);
        account_authority == self.account1.key()
    }
}

impl<'info> Deposit<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[error_code]
pub enum ERC20Error {
    #[msg("Account has insufficient balance")]
    InsufficientBalance,
    #[msg("Yor are trying to transfer more than you allowed to")]
    InsufficientApprove,
    #[msg("You are not authorised for this action")]
    WrongAuthority,
    #[msg("One of the account to wrap is not correct")]
    WrongWrappedToken,
}