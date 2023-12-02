use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize,AnchorSerialize,Clone)]
pub struct CartDetails{
    pub product_key:Pubkey
}

#[account]
pub struct Cart{
    pub user: Pubkey,
    pub cart_details: Vec<CartDetails>
}

#[derive(AnchorDeserialize,AnchorSerialize,Clone)]
pub struct CartData{
    pub user: Pubkey,
    pub cart_details: CartDetails,
}

impl Cart{
    pub const LEN:usize = 500;
}