use anchor_lang::prelude::*;

use crate::{state::{
    milestone::*,
    project::*,
}, error::ProtocolError};

/// Closes a 'Milestone'
pub fn close_milestone(
    ctx: Context<CloseMilestone>,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;
    let milestone = &mut ctx.accounts.milestone;

    milestone.is_active()?;

    if project.admins.contains(&payer_key) {
        milestone.status = MilestoneStatus::Closed;
    } else {
        return Err(ProtocolError::NotAuthorized.into());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CloseMilestone<'info> {
    #[account( 
        seeds = [
            Milestone::SEED_PREFIX.as_bytes(),
            milestone.name.as_ref(),
            milestone.milestone_id.to_le_bytes().as_ref(),
            project.key().as_ref(),
        ],
        bump = milestone.bump,
    )]
    pub milestone: Account<'info, Milestone>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}