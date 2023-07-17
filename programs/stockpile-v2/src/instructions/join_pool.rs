use anchor_lang::prelude::*;

use crate::error::ProtocolError;

use crate::state::{
    pool::*,
    project::*,
};

// Adds a fundraiser into a funding round.
// This is subject to stipulations depending on the pool config.
// i.e: Some pools will require manual approval.
pub fn join_pool(ctx: Context<JoinPool>, _project_id: u64, _pool_id: u64) -> Result<()> {
    let payer_key = ctx.accounts.payer.key();
    let project = &ctx.accounts.project;
    let project_key = ctx.accounts.project.key();
    let mut pool_data = ctx.accounts.pool.clone().into_inner();

    // Fundraiser access control check
    require!(project.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    // Check to make sure the pool is not closed
    ctx.accounts.pool.is_active()?;

    // Check to make sure the fundraiser isnt already in the pool
    if pool_data.project_shares.iter().any(|p| p.project_key == project_key) {
        return Err(ProtocolError::AlreadyEntered.into());
    }

    // Check PoolAccess config
    // For now, Open is the only option exposed.
    if pool_data.pool_access == PoolAccess::Open {

        pool_data.project_shares.push(
            Participant::new(
                project_key, 
    PoolShare::new(),
            )
        );
    } else {
        // In the future, this will be an "else if" to
        // check Manual or Open, and issue a JoinRequest
        // For now, return pool closed due to AIDS
        // https://i.imgur.com/LqFSv8w.jpeg 
        return Err(ProtocolError::PoolClosed.into());
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _project_id: u64,
    _pool_id: u64,
)]
pub struct JoinPool<'info> {
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            _pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project.project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}