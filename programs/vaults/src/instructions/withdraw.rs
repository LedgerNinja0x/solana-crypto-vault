use anchor_lang::prelude::Signer;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    depositor: Signer<'info>,
}

impl<'info> Withdraw<'info> {
    pub fn validate(&self, accounts: &[AccountInfo]) -> Result<()> {
        Ok(())
    }

    pub fn handle(&mut self, accounts: &[AccountInfo], adapter_accounts: Vec<u8>) -> Result<()> {
        Ok(())
    }
}
