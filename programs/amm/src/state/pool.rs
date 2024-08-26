use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // Automatically calculates the space required for the struct.
pub struct Pool {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub k_last: u128,
}

impl Pool {
    /// Initializes the pool with two tokens, ensuring they are in the correct order.
    pub fn initialize(&mut self, token0: Pubkey, token1: Pubkey) -> Result<()> {
        // Ensure token0 is less than token1 to maintain consistent ordering.
        require!(
            token0.to_string() < token1.to_string(),
            ErrorCode::InvalidMintOrder
        );
        self.token0 = token0;
        self.token1 = token1;
        self.k_last = 0;
        Ok(())
    }

    /// Updates the stored product of reserves, k_last, for the pool.
    pub fn update_k_last(&mut self, reserve0: u64, reserve1: u64) {
        self.k_last = reserve0 as u128 * reserve1 as u128;
    }
}
