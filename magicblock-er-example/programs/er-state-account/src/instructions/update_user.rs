use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
}

impl<'info> UpdateUser<'info> {
    pub fn update(&mut self, randomness: [u8; 32]) -> Result<()> {
        let random_data = ephemeral_vrf_sdk::rnd::random_u64(&randomness);

        msg!("Consuming random number: {:?}", random_data);

        self.user_account.data = random_data;

        Ok(())
    }
}
