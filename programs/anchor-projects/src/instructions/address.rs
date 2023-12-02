use anchor_lang::prelude::*;

use crate::{Address, AddressData};

pub fn handle_create_address(ctx:Context<CreateAddress>,address_data:AddressData)->Result<()>{
    let address = &mut ctx.accounts.address;
    address.author = address_data.author;
    address.name = address_data.name;
    address.mobile_no = address_data.mobile_no;
    address.address1 = address_data.address1;
    address.address2 = address_data.address2;
    address.state = address_data.state;
    address.country = address_data.country;
    address.pincode = address_data.pincode;
    Ok(())
}

pub fn handle_update_address(ctx:Context<UpdateAddress>,address_data:AddressData)->Result<()>{
    let address = &mut ctx.accounts.address;
    // address.author = address_data.author;
    address.name = address_data.name;
    address.mobile_no = address_data.mobile_no;
    address.address1 = address_data.address1;
    address.address2 = address_data.address2;
    address.state = address_data.state;
    address.country = address_data.country;
    address.pincode = address_data.pincode;
    Ok(())
}

pub fn handle_delete_address(_ctx:Context<DeleteAddress>)->Result<()>{
    Ok(())
}


#[derive(Accounts)]
pub struct CreateAddress<'info>{
    #[account(init,payer=payer,space=Address::LEN)]
    pub address:Account<'info,Address>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct UpdateAddress<'info>{
    #[account(mut)]
    pub address:Account<'info,Address>,
    // pub payer: Signer<'info>
}


#[derive(Accounts)]
pub struct DeleteAddress<'info>{
    #[account(mut, close=payer)]
    pub address:Account<'info,Address>,
    #[account(mut)]
    pub payer: Signer<'info>
}