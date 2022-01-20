use anchor_lang::prelude::*;

use crate::state::*;


#[derive(Accounts)]
pub struct FooContext<'info> {
    #[account(init, payer = authority, space=8)]
    pub foo_account: Account<'info, FooAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub system_program: Program<'info, System>
}

pub fn handler(_ctx: Context<FooContext>) -> ProgramResult {
    Ok(())
}