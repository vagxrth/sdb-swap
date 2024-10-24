pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FB9JBNRFGMGn3mDPrGbezVxpMaaY1YW7SDDwgmt9LxcF");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault
        ()?;
        instructions::make_offer::save_offer(ctx)
    }
}
