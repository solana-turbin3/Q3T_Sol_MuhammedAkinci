use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::MarketPlaceError, state::Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,

    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(
            name.len() >= 1 && name.len() <= 32,
            MarketPlaceError::NameTooLong
        );

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            name,
            fee,
            rewards_bump: bumps.rewards_mint,
            treasury_bump: bumps.treasury,
            bump: bumps.marketplace,
        });

        Ok(())
    }
}