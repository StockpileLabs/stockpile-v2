use anchor_lang::prelude::*;

use crate::error::ProtocolError;

use crate::state::{
    pool::*,
    project::*,
};

// Adds a fundraiser to a funding round
// This requires the instruction to be called
// by a registered admin of the pool
pub fn add_project(ctx: Context<AddProject>, _project_id: u64, _pool_id: u64) -> Result<()> {
    let payer_key = ctx.accounts.payer.key();
    let project_key = ctx.accounts.project.key();
    let mut pool_data = ctx.accounts.pool.clone().into_inner();

    // Pool access control check
    require!(pool_data.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    // Check to make sure the pool is not closed
    ctx.accounts.pool.is_active()?;

    // Check to make sure the fundraiser isnt already in the pool
    if pool_data.project_shares.iter().any(|p| p.project_key == project_key) {
        return Err(ProtocolError::AlreadyEntered.into());
    }

    if pool_data.pool_access == PoolAccess::Manual {
        pool_data.project_shares.push(
            Participant::new(
                project_key, 
                PoolShare::new(),
            )
        );
    } else {
        // Return an error if the PoolAccess is set to open
        // The fundraiser can just join themselves if they'd like
        return Err(ProtocolError::MismatchedConfig.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _project_id: u64,
    _pool_id: u64,
)]
pub struct AddProject<'info> {
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