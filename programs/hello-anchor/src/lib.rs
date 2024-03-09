use anchor_lang::prelude::*;

declare_id!("Gwo9U4kAXviBuui7VsrdLWJ7y8daf5JZc4BBtHTFh7TP");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
