use anchor_lang::prelude::*;

use crate::state::project::*;
use crate::error::ProtocolError;

/// Closes a 'Project'
pub fn close_project(
    ctx: Context<CloseProject>,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;

    project.is_active()?;

    if project.admins.contains(&payer_key) {
        project.status = ProjectStatus::Closed;
    } else {
        return Err(ProtocolError::NotAuthorized.into());
    };

    Ok(())
}

#[derive(Accounts)]
pub struct CloseProject<'info> {
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