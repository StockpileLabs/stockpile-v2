use anchor_lang::prelude::*;

use crate::error::ProtocolError;
use crate::state::project::*;

pub fn deactivate_project(ctx: Context<DeactivateProject>) -> Result<()> {
    let project = &mut ctx.accounts.project;
    let agent = ctx.accounts.payer.key();

    if project.admins.contains(&agent) {
        project.status = ProjectStatus::Deactivated;
        Ok(())
    } else {
        msg!("Payer is not an admin of this project");
        Err(ProtocolError::NotAuthorized.into())
    }
}

#[derive(Accounts)]
pub struct DeactivateProject<'info> {
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