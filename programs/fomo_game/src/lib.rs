use anchor_lang::prelude::*;

declare_id!("Fomo111111111111111111111111111111111111111");

#[program]
pub mod secure_fomo_vault {
    use super::*;

    /// 初始化合约，仅部署者调用一次
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    /// 只允许通过网页前端（由前端调用的 signer）向合约入金
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let vault = &ctx.accounts.vault;

        require!(user.key() == ctx.accounts.authority.key(), ErrorCode::UnauthorizedDeposit);

        // 将 lamports 从用户转入 vault
        **user.try_borrow_mut_lamports()? -= amount;
        **vault.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    /// 由前端指定转账目标和金额，合约从 vault 执行出金
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let recipient = &ctx.accounts.recipient;

        require!(vault.key() == ctx.accounts.vault.key(), ErrorCode::InvalidVault);

        **vault.try_borrow_mut_lamports()? -= amount;
        **recipient.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // 网页前端用户
    /// CHECK: vault 是合约的资金地址（由程序生成，安全）
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(address = user.key())]
    pub authority: Signer<'info>, // 保证调用者是网页端传入
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: vault 是合约的资金池
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: 目标用户地址（从网页端传入）
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Only the authority can deposit.")]
    UnauthorizedDeposit,
    #[msg("Invalid vault address.")]
    InvalidVault,
}
