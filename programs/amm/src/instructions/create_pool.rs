use crate::state::Config;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreatePool<'info> {
    // Mints of the two tokens that will form the pool.
    pub mint0: Account<'info, Mint>,
    pub mint1: Account<'info, Mint>,

    // Owner of the pool and payer for the transaction.
    #[account(mut)]
    pub owner: Signer<'info>,

    // Configuration account with specific constraints (owner and bump).
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = owner)]
    pub config: Account<'info, Config>,

    // The pool account being created, with seeds for uniqueness.
    #[account(init, seeds= [b"pool", mint0.key().as_ref(), mint1.key().as_ref()], bump, payer = owner, space = 8 + Pool::INIT_SPACE)]
    pub pool: Box<Account<'info, Pool>>,

    // Authority derived from the pool account, used to control associated PDAs.
    /// CHECK: authority so 1 acc pass in can derive all other PDAs
    #[account(seeds=[b"authority", pool.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    // Associated token accounts for each token in the pool, tied to the pool's authority.
    #[account(
        associated_token::mint = mint0,
        associated_token::authority = pool_authority
    )]
    pub vault0: Box<Account<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint1,
        associated_token::authority = pool_authority
    )]
    pub vault1: Box<Account<'info, TokenAccount>>,

    // Mint for the liquidity provider (LP) tokens, controlled by the pool's authority.
    #[account(init, payer = owner, seeds = [b"lp_mint", pool.key().as_ref()], bump, mint::decimals = 6, mint::authority = pool_authority)]
    pub lp_mint: Box<Account<'info, Mint>>,

    // Required programs and system accounts.
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

// Define the event for pool creation.
#[event]
pub struct PoolCreated {
    pub pool: Pubkey,
    pub mint0: Pubkey,
    pub mint1: Pubkey,
}

pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    // Initialize the pool with the provided token mints.
    pool.initialize(ctx.accounts.mint0.key(), ctx.accounts.mint1.key())?;

    // Emit the PoolCreated event.
    emit!(PoolCreated {
        pool: pool.key(),
        mint0: ctx.accounts.mint0.key(),
        mint1: ctx.accounts.mint1.key(),
    });

    Ok(())
}
