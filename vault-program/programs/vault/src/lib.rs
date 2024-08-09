use anchor_lang::prelude::*;

declare_id!("9dxMaDdRW6Yu55BVPWDdjiSBZV39hFUhJmfP3LHg2Lh2");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
