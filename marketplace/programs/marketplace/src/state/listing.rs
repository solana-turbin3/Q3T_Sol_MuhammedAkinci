use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub maker: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}