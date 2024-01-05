use anchor_lang::prelude::*;

use crate::{state::pool::*, error::ProtocolError};

/// Updates a pool, given a specific field
pub fn update_pool(
    ctx: Context<UpdatePool>,
    _pool_id: u64,
    update_field: UpdatePoolField,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let pool = &mut ctx.accounts.pool;

    require!(pool.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    match update_field {
        UpdatePoolField::Name(new_name) => pool.name = new_name,
        UpdatePoolField::AddAdmin(new_admin) => {
            if !pool.admins.contains(&new_admin) {
                pool.admins.push(new_admin);
            }
        },
        UpdatePoolField::RemoveAdmin(admin_to_remove) => {
            pool.admins.retain(|&admin| admin != admin_to_remove);
        },
        UpdatePoolField::Approval(new_approval) => pool.pool_access = new_approval,
        UpdatePoolField::Status(new_status) => pool.pool_state = new_status,
    };

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _pool_id: u64,
)]
pub struct UpdatePool<'info> {
    #[account(
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            _pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}