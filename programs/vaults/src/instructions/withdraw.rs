use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_lang::prelude::{Account, Signer};

#[derive(Accounts)]
pub struct Withdraw<'info> {

}

impl<'info> Withdraw<'info> {
    pub fn validate(self) -> Result<()> {
        Ok(())
    }

    pub fn handle(&mut self) -> Result<()> {
        Ok(())
    }
}