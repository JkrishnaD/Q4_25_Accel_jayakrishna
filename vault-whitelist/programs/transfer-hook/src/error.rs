use anchor_lang::prelude::*;

#[error_code]
pub enum TransferHookErrors {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Whitelist PDA does not match")]
    WhitelistPdaMismatch,
    #[msg("Account is not whitelisted")]
    AccountNotWhitelisted,
}
