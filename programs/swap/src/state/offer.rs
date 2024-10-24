use anchor_lang::prelude::{Pubkey, *};

#[account]
#[derive(InitSpace)]
pub struct Offer {  
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_amount_need: u64,
    pub bump: u8
}