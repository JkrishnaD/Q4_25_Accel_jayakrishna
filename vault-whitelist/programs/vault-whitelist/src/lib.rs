pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EXCveHWMKpKACJo6Xe2jU1gXhWAmwPpBAiGNV2yaBEW3");

#[program]
pub mod vault_whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
