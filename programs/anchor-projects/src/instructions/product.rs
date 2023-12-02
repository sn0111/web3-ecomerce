use anchor_lang::prelude::*;
use crate::state::*;

pub fn handle_create_product(ctx: Context<CreateProduct>,product: Data) -> Result<()> {
    let p = &mut ctx.accounts.product;

    p.author = ctx.accounts.payer.key();
    p.name = product.name;
    p.quantity = product.quantity;
    p.desc = product.desc;
    p.image_url = product.image_url;
    p.cost = product.cost;
    p.mrp_amount = product.mrp_amount;
    p.discount = product.discount;
    
    Ok(())
}

pub fn handle_update_product(ctx: Context<UpdateProduct>,data:Data)->Result<()>{
    let p = &mut ctx.accounts.product;
    p.name = data.name;
    p.quantity = data.quantity;
    p.desc = data.desc;
    p.image_url = data.image_url;
    p.cost = data.cost;
    p.mrp_amount = data.mrp_amount;
    p.discount = data.discount;
    Ok(())
}

pub fn handle_delete_product(_ctx: Context<DeleteProduct>) -> Result<()>{
    Ok(())
}



#[derive(Accounts)]
pub struct CreateProduct<'info> {
    #[account(init,payer=payer,space=Product::LEN)]
    pub product: Account<'info,Product>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct UpdateProduct<'info>{
    #[account(mut)]
    pub product: Account<'info,Product>
}

#[derive(Accounts)]
pub struct DeleteProduct<'info>{
    #[account(mut, close=payer)]
    pub product: Account<'info,Product>,
    #[account(mut)]
    pub payer: Signer<'info>
}
