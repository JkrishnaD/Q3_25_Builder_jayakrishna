use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

declare_id!("4GVLkDuTPnMCiw7nuDrwLiGy6YzxCczxD1T4vnF6wRNM");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposite(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposite(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close_account(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,seeds=[b"vault",user.key().as_ref()],bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,

    #[account(mut,seeds=[b"state",user.key().as_ref()],bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposite(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let user_key = self.user.key();
        let vault_bump = self.vault_state.vault_bump;
        let seeds = [b"vault", user_key.as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault",user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[b"state",user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let vault_balance = self.vault.to_account_info().lamports();

        if vault_balance > 0 {
            let cpi_program = self.system_program.to_account_info();

            let cpi_account = Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            };

            let user_key = self.user.key();
            let vault_bump = self.vault_state.vault_bump;
            let seeds = [b"vault", user_key.as_ref(), &[vault_bump]];

            let signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, signer);

            transfer(cpi_ctx, vault_balance)?;
        }
        Ok(())
    }
}
