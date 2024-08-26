use anchor_lang::prelude::*;

declare_id!("HCr5iuS7BuDBPUW2S53ShHAgpYUsUrodDLV8wrWg2WjW");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
