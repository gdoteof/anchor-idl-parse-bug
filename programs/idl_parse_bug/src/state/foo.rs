use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize, Copy)]
pub enum MyType {
    Bar,
    Baz
}

#[account]
pub struct FooAccount {
    pub mine: MyType,
}