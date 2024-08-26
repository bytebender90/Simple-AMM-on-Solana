use anchor_lang::prelude::*;
use instructions::*;

pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("4sRbFuajHVG181psKiK7G2JBSzbcvVD9RBVbo72DE9TQ");

#[program]
pub mod amm {
    use super::*;

    /// Initializes the AMM with the provided fee recipient and fee amount.
    pub fn initialize(ctx: Context<Initialize>, fee_to: Pubkey, fee: u64) -> Result<()> {
        instructions::initialize(ctx, fee_to, fee)
    }

    /// Updates the fee recipient address.
    pub fn set_fee_to(ctx: Context<SetFeeTo>, new_fee_to: Pubkey) -> Result<()> {
        instructions::set_fee_to(ctx, new_fee_to)
    }

    /// Updates the fee amount.
    pub fn set_fee(ctx: Context<SetFee>, new_fee: u64) -> Result<()> {
        instructions::set_fee(ctx, new_fee)
    }

    /// Creates a new liquidity pool.
    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    /// Adds liquidity to the pool, specifying desired and minimum amounts.
    pub fn add_liquidity(
        ctx: Context<LiquidityOperation>,
        amount0_desired: u64,
        amount1_desired: u64,
        amount0_min: u64,
        amount1_min: u64,
    ) -> Result<()> {
        instructions::add_liquidity(
            ctx,
            amount0_desired,
            amount1_desired,
            amount0_min,
            amount1_min,
        )
    }

    /// Removes liquidity from the pool, specifying the amount and minimum outputs.
    pub fn remove_liquidity(
        ctx: Context<LiquidityOperation>,
        liquidity: u64,
        amount0_min: u64,
        amount1_min: u64,
    ) -> Result<()> {
        instructions::remove_liquidity(ctx, liquidity, amount0_min, amount1_min)
    }

    /// Swaps an exact input amount for a minimum output amount.
    pub fn swap_exact_input(
        ctx: Context<Swap>,
        input_amount: u64,
        min_output_amount: u64,
    ) -> Result<()> {
        instructions::swap_exact_input(ctx, input_amount, min_output_amount)
    }

    /// Swaps to obtain an exact output amount, specifying a maximum input amount.
    pub fn swap_exact_output(
        ctx: Context<Swap>,
        output_amount: u64,
        max_input_amount: u64,
    ) -> Result<()> {
        instructions::swap_exact_output(ctx, output_amount, max_input_amount)
    }
}
