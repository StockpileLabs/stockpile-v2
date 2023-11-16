use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::error::ProtocolError;
use crate::state::{
    pool::*,
    project::*,
};

/// Claim's a project's payout from a pool
/// that has completed.
pub fn claim_payout(
    ctx: Context<ClaimPayout>,
    _project_id: u64,
    _pool_id: u64,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;
    let current_time = Clock::get()?.unix_timestamp as u64;

    let mut pool_data = ctx.accounts.pool.clone().into_inner();

    project.is_active()?;

    if current_time > ctx.accounts.pool.end {
        if project.admins.contains(&payer_key) {
            if let Some(participant) = pool_data.project_shares.iter_mut().find(|p| p.project_key == project.key()) {
                let payout = participant.share_data.share * ctx.accounts.pool.total_funding as f64;

                let mut adj_payout = payout as u64;

                msg!("Payout for {:?} is {:?}", project.key(), payout);

                adj_payout /= 10_u64.pow(6);

                token::transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        token::Transfer {
                            from: ctx.accounts.pool_token_account.to_account_info(),
                            to: ctx.accounts.project_token_account.to_account_info(),
                            authority: ctx.accounts.payer.to_account_info(),
                        },
                    ),
                    adj_payout,
                )?;

                participant.claimed = true;
            } else {
                return Err(ProtocolError::NotInPool.into());
            };
        } else {
            return Err(ProtocolError::NotAuthorized.into());
        };
    } else {
        return Err(ProtocolError::PoolStillActive.into());
    };

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    project_id: u64,
)]
pub struct ClaimPayout<'info> {
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project.project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool.pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    pub mint: Account<'info, token::Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = project,
    )]
    pub project_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = project,
    )]
    pub pool_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}