mod state;

use anchor_lang::prelude::*;

declare_id!("GUu6UhR3Bpg2RrVSDPbXjcjbHuHHFsxSh8BkypArVU2Q");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
