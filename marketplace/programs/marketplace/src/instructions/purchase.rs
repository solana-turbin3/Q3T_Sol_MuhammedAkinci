use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    maker: SystemAccount<'info>,
    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = maker_mint
    )]
    taker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::authority = listing,
        associated_token::mint = maker_mint
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump
    )]
    listing: Box<Account<'info, Listing>>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    tresury: SystemAccount<'info>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol_to_maker(&mut self) -> Result<()> {
        let cpi_accounts = system_program::Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, self.listing.price)?;

        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();

        let cpi_accounts = system_program::Transfer {
            from: self.taker.to_account_info(),
            to: self.tresury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, fee)
    }

    pub fn transfer_nft_to_taker(&mut self) -> Result<()> {
        let cpi_accounts = token_interface::TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        token_interface::transfer_checked(cpi_ctx, 1, 0)
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let cpi_accounts = token_interface::CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        token_interface::close_account(cpi_ctx)
    }
}