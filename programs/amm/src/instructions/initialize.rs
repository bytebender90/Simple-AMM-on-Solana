use crate::state::config::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // The owner who will pay for the account creation and initialization.
    #[account(mut)]
    pub owner: Signer<'info>,

    // The config account being initialized, with a unique seed and calculated space.
    #[account(init, payer = owner, seeds = [b"config"], bump, space = 8 + Config::INIT_SPACE)]
    pub config: Account<'info, Config>,

    // Required system program for account creation.
    pub system_program: Program<'info, System>,

    // Rent sysvar, needed to determine rent-exempt status.
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct ConfigInitialized {
    pub owner: Pubkey,
    pub fee_to: Pubkey,
    pub fee: u64,
}

pub fn initialize(ctx: Context<Initialize>, fee_to: Pubkey, fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // Initialize the config with the given parameters.
    config.initialize(ctx.bumps.config, *ctx.accounts.owner.key, fee_to, fee)?;

    // Emit an event to log the initialization.
    emit!(ConfigInitialized {
        owner: *ctx.accounts.owner.key,
        fee_to,
        fee,
    });

    Ok(())
}
