use crate::constant::BASIS_POINTS;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // Automatically calculates the space required for the struct.
pub struct Config {
    pub bump: u8,
    pub owner: Pubkey,
    pub fee_to: Pubkey,
    pub fee: u64,
}

impl Config {
    /// Initializes the config with the specified parameters, ensuring the fee is valid.
    pub fn initialize(&mut self, bump: u8, owner: Pubkey, fee_to: Pubkey, fee: u64) -> Result<()> {
        // Ensure the fee is less than the maximum allowed value.
        require!(fee < BASIS_POINTS, ErrorCode::InvalidFee);

        self.bump = bump;
        self.owner = owner;
        self.fee_to = fee_to;
        self.fee = fee;

        Ok(())
    }

    /// Sets a new fee, ensuring it is valid.
    pub fn set_fee(&mut self, fee: u64) -> Result<()> {
        // Ensure the fee is less than the maximum allowed value.
        require!(fee < BASIS_POINTS, ErrorCode::InvalidFee);

        self.fee = fee;
        Ok(())
    }

    /// Updates the fee recipient.
    pub fn set_fee_to(&mut self, fee_to: Pubkey) -> Result<()> {
        self.fee_to = fee_to;
        Ok(())
    }
}
