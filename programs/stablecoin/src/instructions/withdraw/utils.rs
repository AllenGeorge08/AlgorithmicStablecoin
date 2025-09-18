use crate::constants::{FEED_ID, MAXIUMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT, SEED_SOL_ACCOUNT};
use crate::errors::CustomError;
use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token::{burn, Burn},
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn withdraw_sol<'info>(
    bump: &u8,
    depositor_key: &Pubkey,
    system_program: &Program<'info, System>,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[*bump]]]; //e Cool, we wanted a u8 inside but & outside...

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?; //e System program transfer

    Ok(())
}

pub fn burn_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    mint_account: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                //e Burn instruction
                mint: mint_account.to_account_info(), //e The mint_account is the account that holds the authority of the token like mint/burn etc..
                from: token_account.to_account_info(), //e Token coming from token account of the user...
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
