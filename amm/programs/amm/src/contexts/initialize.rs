use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked, 
        transfer_checked,
        mint_to,
        MintTo
    }
};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + Config::INIT_SPACE,
        seeds = [
            b"amm", 
            mint_x.key().as_ref(), 
            mint_y.key().as_ref(), 
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    config: Box<Account<'info, Config>>,
    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"mint", config.key().as_ref()],
        bump,
    )]
    mint_lp: Box<InterfaceAccount<'info, Mint>>,
    mint_x: Box<InterfaceAccount<'info, Mint>>,
    mint_y: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    vault_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    vault_y: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_y: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn save_config(
        &mut self, 
        seed: u64, 
        fee: u16, 
        bump: u8, 
        lp_bump: u8
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            fee,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            lp_bump,
            bump
        });

        Ok(())
    }

    pub fn deposit(
        &self,
        amount: u64,
        is_x: bool
    ) -> Result<()> {

        let (
            from, 
            to, 
            mint,
            decimals
        ) = match is_x {
            true => {(
                    self.maker_ata_x.to_account_info(),
                    self.vault_x.to_account_info(),
                    self.mint_x.to_account_info(),
                    self.mint_x.decimals
            )},
            false => {(
                    self.maker_ata_y.to_account_info(),
                    self.vault_y.to_account_info(),
                    self.mint_y.to_account_info(),
                    self.mint_y.decimals
            )}
        };

        let accounts = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.maker.to_account_info()
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            accounts
        );

        transfer_checked(
            ctx, 
            amount,
            decimals
        )
    }

    pub fn mint_lp_tokens(
        &self,
        amount_x: u64,
        amount_y: u64
    ) -> Result<()> {
        let amount = amount_x.checked_mul(amount_y).ok_or(ProgramError::ArithmeticOverflow)?;

        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seed = self.config.seed.to_le_bytes();
        let bump = [self.config.bump];
        let mint_a = self.mint_x.to_account_info().key();
        let mint_b = self.mint_y.to_account_info().key();
        let signer_seeds = [&[
            b"amm",
            mint_a.as_ref(),
            mint_b.as_ref(),
            seed.as_ref(),
            &bump
        ][..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            &signer_seeds
        );

        mint_to(ctx, amount)

    }
}