use anchor_lang::prelude::*;

use crate::{state::pool::*, error::ProtocolError};

/// Extends the duration of the pool by
/// updating it's end field with a new timestamp.
/// This instruction can only extend a pool, not shorten.
pub fn extend_pool_duration(
    ctx: Context<ExtendPool>,
    _pool_id: u64,
    new_end_date: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let payer = &ctx.accounts.payer;

    require!(pool.admins.contains(&payer.key()), ProtocolError::NotAuthorized);

    pool.extend_pool_duration(new_end_date)
    .expect("Failed to update end date.");
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    _new_end_date: u64,
)]
pub struct ExtendPool<'info> {
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}