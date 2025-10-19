use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = 8 + ExtraAccountMetaList::size_of(1)?,
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas(address: Pubkey) -> Result<Vec<ExtraAccountMeta>> {
        let (whitelist_pubkey, _) = Pubkey::find_program_address(
            &[b"whitelist", address.key().as_ref()],
            &vault_whitelist::ID,
        );
        Ok(vec![ExtraAccountMeta::new_with_pubkey(
            &whitelist_pubkey,
            false,
            true,
        )?])
    }
}
