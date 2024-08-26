use crate::state::config::Config;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo};
use fixed::types::U128F0;

#[event]
pub struct FeeToSet {
    pub old_fee_to: Pubkey,
    pub new_fee_to: Pubkey,
}

#[event]
pub struct FeeSet {
    pub old_fee: u64,
    pub new_fee: u64,
}

#[event]
pub struct LiquidityMinted {
    pub liquidity: u64,
}

#[derive(Accounts)]
pub struct SetFeeTo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner of the contract
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = owner)]
    pub config: Account<'info, Config>, // The configuration account
    pub system_program: Program<'info, System>,
}

pub fn set_fee_to(ctx: Context<SetFeeTo>, new_fee_to: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let old_fee_to = config.fee_to;

    config.set_fee_to(new_fee_to)?;
    emit!(FeeToSet {
        old_fee_to,
        new_fee_to,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner of the contract
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = owner)]
    pub config: Account<'info, Config>, // The configuration account
    pub system_program: Program<'info, System>,
}

pub fn set_fee(ctx: Context<SetFee>, new_fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let old_fee = config.fee;

    config.set_fee(new_fee)?;
    emit!(FeeSet { old_fee, new_fee });

    Ok(())
}

pub fn mint_fee<'info>(
    _config: &Config,
    pool: &Pool,    // The liquidity pool
    reserve0: u64,  // Reserve of token0
    reserve1: u64,  // Reserve of token1
    lp_supply: u64, // Total supply of liquidity tokens
    mint_ctx: CpiContext<'_, '_, '_, 'info, MintTo<'info>>,
) -> Result<()> {
    let k_last = pool.k_last;

    if k_last != 0 {
        let root_k: u128 = U128F0::from_num((reserve0 as u128) * (reserve1 as u128))
            .sqrt()
            .to_num::<u128>();
        let root_k_last = U128F0::from_num(k_last).sqrt().to_num::<u128>();
        if root_k > root_k_last {
            let numerator: u128 = (lp_supply as u128) * (root_k - root_k_last);
            let denominator: u128 = root_k * 5 + root_k_last;
            let liquidity: u64 = (numerator / denominator) as u64;
            if liquidity > 0 {
                mint_to(mint_ctx, liquidity)?; // Mint new liquidity tokens
                emit!(LiquidityMinted { liquidity });
            }
        }
    }

    Ok(())
}
