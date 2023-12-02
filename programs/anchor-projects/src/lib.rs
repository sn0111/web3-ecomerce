use anchor_lang::prelude::*;
pub use state::*;
pub use instructions::*;
mod state;
mod instructions;

declare_id!("9HhY4MdAGfRychTsnhGXbfEmXJ4vzjboKS26aduQcf5i");

#[program]
pub mod anchor_projects {

    use super::*;

    pub fn create_product(ctx: Context<CreateProduct>,product_data: Data) -> Result<()> {
        handle_create_product(ctx, product_data)
    }

    pub fn update_product(ctx: Context<UpdateProduct>,data:Data)->Result<()>{
        handle_update_product(ctx, data)
    }

    pub fn delete_product(ctx: Context<DeleteProduct>) -> Result<()>{
        handle_delete_product(ctx)
    }

    pub fn create_address(ctx: Context<CreateAddress>,product_data: AddressData) -> Result<()> {
        handle_create_address(ctx, product_data)
    }

    pub fn update_address(ctx: Context<UpdateAddress>,data:AddressData)->Result<()>{
        handle_update_address(ctx, data)
    }

    pub fn delete_address(ctx: Context<DeleteAddress>) -> Result<()>{
        handle_delete_address(ctx)
    }

    pub fn transfer(ctx: Context<Tranfer>,amount:u64) -> Result<()>{
        handle_transfer(ctx,amount)
    }

}



