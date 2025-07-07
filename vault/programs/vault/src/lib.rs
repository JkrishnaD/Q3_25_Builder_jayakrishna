use anchor_lang::prelude::*;

declare_id!("4GVLkDuTPnMCiw7nuDrwLiGy6YzxCczxD1T4vnF6wRNM");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
}