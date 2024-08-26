use crate::constant::BASIS_POINTS;
use crate::error::ErrorCode;
use crate::state::Config;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Swap<'info> {
    // Config PDA
    #[account( seeds = [b"config"], bump = config.bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    // User's source token account, must be owned by the user
    #[account(mut, has_one = owner)]
    pub user_ata_src: Box<Account<'info, TokenAccount>>,

    // User's destination token account, must be owned by the user
    #[account(mut, has_one = owner)]
    pub user_ata_des: Box<Account<'info, TokenAccount>>,

    // Pool account
    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,

    // Pool authority PDA
    /// CHECK: authority so 1 acc pass in can derive all other PDAs
    #[account(seeds=[b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    // Source vault for the swap, must match the mint of the user's source token account
    #[account(mut, constraint = user_ata_src.mint == vault_src.mint)]
    pub vault_src: Box<Account<'info, TokenAccount>>,

    // Destination vault for the swap, must match the mint of the user's destination token account
    #[account(mut, constraint = user_ata_des.mint == vault_des.mint)]
    pub vault_des: Box<Account<'info, TokenAccount>>,

    // LP mint PDA
    #[account(mut, seeds = [b"lp_mint", pool.key().as_ref()], bump)]
    pub lp_mint: Box<Account<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct SwapEvent {
    pub owner: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub vault_src: Pubkey,
    pub vault_des: Pubkey,
}

pub fn swap_exact_input(
    ctx: Context<Swap>,
    input_amount: u64,
    min_output_amount: u64,
) -> Result<()> {
    // Calculate the output amount based on the input
    let amount_out = get_amount_out(
        &ctx.accounts.config,
        input_amount,
        ctx.accounts.vault_src.amount,
        ctx.accounts.vault_des.amount,
    )?;

    // Ensure the output amount meets the minimum required
    require!(
        amount_out >= min_output_amount,
        ErrorCode::InsufficientOutputAmount,
    );

    // Execute the swap
    swap(ctx, input_amount, amount_out)?;

    Ok(())
}

pub fn swap_exact_output(
    ctx: Context<Swap>,
    output_amount: u64,
    max_input_amount: u64,
) -> Result<()> {
    // Calculate the required input amount to get the desired output
    let amount_in = get_amount_in(
        &ctx.accounts.config,
        output_amount,
        ctx.accounts.vault_src.amount,
        ctx.accounts.vault_des.amount,
    )?;

    // Ensure the input amount does not exceed the maximum allowed
    require!(
        amount_in <= max_input_amount,
        ErrorCode::InsufficientInputAmount,
    );

    // Execute the swap
    swap(ctx, amount_in, output_amount)?;

    Ok(())
}

fn get_amount_out(
    config: &Config,
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
) -> Result<u64> {
    // Ensure there is sufficient liquidity in both reserves
    require!(
        reserve_in > 0 && reserve_out > 0,
        ErrorCode::InsufficientLiquidity,
    );

    // Calculate output amount with fee applied
    let amount_in_with_fee = amount_in as u128 * (BASIS_POINTS - config.fee) as u128;
    let numerator = amount_in_with_fee * reserve_out as u128;
    let denominator = reserve_in as u128 * BASIS_POINTS as u128 + amount_in_with_fee;

    Ok((numerator / denominator) as u64)
}

fn get_amount_in(
    config: &Config,
    amount_out: u64,
    reserve_in: u64,
    reserve_out: u64,
) -> Result<u64> {
    // Ensure there is sufficient liquidity in both reserves
    require!(
        reserve_in > 0 && reserve_out > 0,
        ErrorCode::InsufficientLiquidity,
    );

    // Calculate input amount required to get the desired output
    let numerator = reserve_in as u128 * amount_out as u128 * BASIS_POINTS as u128;
    let denominator =
        (reserve_out as u128 - amount_out as u128) * (BASIS_POINTS - config.fee) as u128;

    Ok((numerator / denominator + 1) as u64)
}

fn swap(ctx: Context<Swap>, input_amount: u64, output_amount: u64) -> Result<()> {
    // Ensure valid amounts for swap
    require!(output_amount > 0, ErrorCode::InsufficientOutputAmount,);
    require!(input_amount > 0, ErrorCode::InsufficientInputAmount,);
    require!(
        output_amount < ctx.accounts.vault_des.amount,
        ErrorCode::InsufficientLiquidity,
    );
    require!(
        input_amount < ctx.accounts.user_ata_src.amount,
        ErrorCode::InsufficientUserBalance,
    );

    let pool: &Box<Account<Pool>> = &ctx.accounts.pool;
    let pool_key = pool.key();
    let pool_sign = &[b"authority", pool_key.as_ref(), &[ctx.bumps.pool_authority]];

    // Transfer tokens from the user's source account to the vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_ata_src.to_account_info(),
                to: ctx.accounts.vault_src.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        input_amount,
    )?;

    // Transfer tokens from the vault to the user's destination account
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_des.to_account_info(),
                to: ctx.accounts.user_ata_des.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
        )
        .with_signer(&[pool_sign]),
        output_amount,
    )?;

    // Emit event after successful swap
    emit!(SwapEvent {
        owner: ctx.accounts.owner.key(),
        input_amount,
        output_amount,
        vault_src: ctx.accounts.vault_src.key(),
        vault_des: ctx.accounts.vault_des.key(),
    });

    Ok(())
}
