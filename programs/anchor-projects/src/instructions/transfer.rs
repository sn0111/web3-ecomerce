use anchor_lang::{prelude::*, solana_program::{program::invoke_signed, system_instruction}};

pub fn handle_transfer(ctx:Context<Tranfer>, amount: u64)->Result<()>{
    invoke_signed(
        &system_instruction::transfer(ctx.accounts.from.key, ctx.accounts.to.key, amount), 
        &[ctx.accounts.from.to_account_info(),ctx.accounts.to.clone(),ctx.accounts.system_program.to_account_info()], 
        &[]
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct Tranfer<'info>{
    #[account(mut)]
    pub from:Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub to:AccountInfo<'info>,
    pub system_program: Program<'info,System>
}