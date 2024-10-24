use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TransferChecked}, token_interface::{TokenAccount, TokenInterface}};

pub fn transfer_tokens<'info> ( 
    from: InterfaceAccount<'info, TokenAccount>,
    to: InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface> 
    ) -> Result<()> {
        let transfer_accounts_options = TransferChecked {
            from: from.to_account_info(),
            mint: mint.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info()
        };
}   