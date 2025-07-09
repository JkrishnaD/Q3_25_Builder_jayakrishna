use anchor_lang::prelude::*;

declare_id!("CwgCa5b6vwB5DLRdtAc8rQ9gFhqUvBEoT5mk1Uh3zLWd");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
