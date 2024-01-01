use anchor_lang::prelude::*;

use crate::{state::project::*, error::ProtocolError};

/// Updates a project, given a specific field
pub fn update_project(
    ctx: Context<UpdateProject>,
    _project_id: u64,
    update_field: UpdateField,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;

    require!(project.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    match update_field {
        UpdateField::Name(new_name) => project.name = new_name,
        UpdateField::Goal(new_goal) => project.goal = new_goal,
        UpdateField::AddAdmin(new_admin) => {
            if !project.admins.contains(&new_admin) {
                project.admins.push(new_admin);
            }
        },
        UpdateField::RemoveAdmin(admin_to_remove) => {
            project.admins.retain(|&admin| admin != admin_to_remove);
        },
        UpdateField::Beneficiary(new_beneficiary) => project.beneficiary = new_beneficiary,
        UpdateField::Status(new_status) => project.status = new_status,
    };

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _project_id: u64,
)]
pub struct UpdateProject<'info> {
    #[account(
        mut,
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            _project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}