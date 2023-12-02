use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
pub struct Product{
    pub author: Pubkey,
    pub name: String,
    pub quantity: u64,
    pub desc: String,
    pub image_url: String,
    pub cost: u64,
    pub mrp_amount: u64,
    pub discount: u8
}

#[derive(AnchorDeserialize,AnchorSerialize,Clone)]
pub struct Data{
    pub author: Pubkey,
    pub name: String,
    pub quantity: u64,
    pub desc: String,
    pub image_url: String,
    pub cost: u64,
    pub mrp_amount: u64,
    pub discount: u8
}

impl Product{
    pub const LEN:usize = 8 + 4 + 32 + 32 + 8 + 4 + 50*4 + 4 + 100*4 + 8 + 8 + 1;
}