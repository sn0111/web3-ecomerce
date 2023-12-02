use anchor_lang::prelude::*;


#[account]
pub struct Address{
    pub author: Pubkey,
    pub name: String,
    pub mobile_no: String,
    pub address1: String,
    pub address2: String,
    pub state: String,
    pub country: String,
    pub pincode: String 
}

#[derive(AnchorDeserialize,AnchorSerialize,Clone)]
pub struct AddressData{
    pub author: Pubkey,
    pub name: String,
    pub mobile_no: String,
    pub address1: String,
    pub address2: String,
    pub state: String,
    pub country: String,
    pub pincode: String 
}

impl Address {
    pub const LEN:usize = 8+32+4+35+4+12+4+100+4+100+4+20+4+20+4+10;
}