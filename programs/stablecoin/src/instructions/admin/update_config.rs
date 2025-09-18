use crate::{
    Config, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR,
    SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds  = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, Config>,
}

pub fn process_update_config(ctx: Context<UpdateConfig>, new_min_health_factor: u64) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.min_health_factor = new_min_health_factor;

    Ok(())
}
