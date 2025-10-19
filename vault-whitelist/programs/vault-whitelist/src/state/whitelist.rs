use anchor_lang::prelude::*;

#[account]
pub struct Whitelist{
    pub whitelist:Vec<(Pubkey,u64)>,
    pub whitelist_bump:u8,
}

impl Whitelist {

}
