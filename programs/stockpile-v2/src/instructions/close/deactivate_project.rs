use anchor_lang::prelude::*;

pub fn deactivate_project(ctx: Context<DeactivateProject>) -> Result<()> {
    let project = ctx.accounts.project;
    let agent = ctx.accounts.payer.key();

    if project.admins.contains(&agent) {
        project.status = ProjectStatus::Inactive;
        Ok(())
    } else {
        msg!("Payer is not an admin of this project");
        Err(ErrorCode::NotAuthorized.into())
    }
}

#[derive(Accounts)]
pub struct DeactivateProject<'info> {
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}