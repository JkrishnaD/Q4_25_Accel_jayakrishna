use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};
use vault_whitelist::Whitelist;

use crate::error::TransferHookErrors;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK : source token account owner can be the system account or any PDA
    pub owner: UncheckedAccount<'info>,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist",owner.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

impl<'info> TransferHook<'info> {
    pub fn transfer_hook(&mut self) -> Result<()> {
        self.check_if_transfering()?;

        let (whitelist_pda, _) = Pubkey::find_program_address(
            &[b"whitelist", self.owner.key().as_ref()],
            &vault_whitelist::ID,
        );

        require_keys_eq!(
            whitelist_pda,
            self.whitelist.key(),
            TransferHookErrors::WhitelistPdaMismatch
        );
        require!(
            !self.whitelist.is_whitelisted,
            TransferHookErrors::AccountNotWhitelisted
        );

        Ok(())
    }

    fn check_if_transfering(&mut self) -> Result<()> {
        // Getting the information about the source token account
        let source_token_info = self.source_token.to_account_info();

        // Borrow the mutable data of the source token account
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;

        // Unpack the account data as a PodStateWithExtensionsMut
        // This will allow us to access the extensions of the token account
        // We use PodStateWithExtensionsMut because TokenAccount is a POD (Plain Old Data) type
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;

        // Get the TransferHookAccount extension
        // Search for the TransferHookAccount extension in the token account
        // The returning struct has a `transferring` field that indicates if the account is in the middle of a transfer operation
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        // Check if the account is in the middle of a transfer operation
        if !bool::from(account_extension.transferring) {
            panic!("TransferHook: Not transferring");
        }
        Ok(())
    }
}
