use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault{
    pub owner:Pubkey,
    pub mint:Pubkey,
    pub vault_bump:u64,
}
