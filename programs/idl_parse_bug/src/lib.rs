use anchor_lang::{prelude::*};

pub mod state;
pub mod instructions;

use instructions::foo::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod idl_parse_bug {
    use super::*;

    pub fn foo(ctx: Context<FooContext>) -> ProgramResult {
        instructions::foo::handler(ctx)
    }
}
