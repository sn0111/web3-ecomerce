use anchor_lang::prelude::*;
use crate::state::*;

pub fn handle_create_cart(ctx: Context<CreateCart>,cart: CartData) -> Result<()> {
    let p = &mut ctx.accounts.product;

    let cart_details:Vec<CartDetails> = Vec::new();

    p.user = cart.user;
    p.cart_details = cart_details;
    
    Ok(())
}

pub fn handle_add_cart(ctx: Context<AddCart>,data:CartDetails)->Result<()>{
    let p = &mut ctx.accounts.product;
    let cart_details =CartDetails{product_key:data.product_key};
    p.cart_details.push(cart_details);
    Ok(())
}

pub fn handle_delete_cart(ctx: Context<DeleteCart>,product_key:Pubkey) -> Result<()>{
    let p = &mut ctx.accounts.product;
    let index = p.cart_details.iter().position(|x| x.product_key==product_key).unwrap();
    p.cart_details.remove(index);
    Ok(())
}



#[derive(Accounts)]
pub struct CreateCart<'info> {
    #[account(init,payer=payer,space=Product::LEN)]
    pub product: Account<'info,Cart>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct AddCart<'info>{
    #[account(mut)]
    pub product: Account<'info,Cart>
}

#[derive(Accounts)]
pub struct DeleteCart<'info>{
    #[account(mut)]
    pub product: Account<'info,Cart>
}

// #[derive(Accounts)]
// pub struct DeleteCart<'info>{
//     #[account(mut, close=payer)]
//     pub product: Account<'info,Cart>,
//     #[account(mut)]
//     pub payer: Signer<'info>
// }
