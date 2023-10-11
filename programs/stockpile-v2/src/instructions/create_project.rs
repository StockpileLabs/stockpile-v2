use anchor_lang::prelude::*;

use crate::state::project::*;

/// Creates a `Project` for a project.
/// This project can be re-entered in funding rounds and
/// only needs to be created as a `Project` once.
pub fn create_project(
    ctx: Context<CreateProject>,
    project_id: u64,
    name: String,
    mut admins: Vec<Pubkey>,
    beneficiary: Pubkey,
    goal: u64,
) -> Result<()> {

    if admins.len() == 0 {
        //Add beneficiary to admin vec
        admins.push(ctx.accounts.payer.key());

        ctx.accounts.project.set_inner(
            Project::new(
                project_id,
                name,
                admins,
                goal,
                beneficiary,
                *ctx.bumps
                    .get("project")
                    .expect("Failed to derive bump for `project`"),
            )?
        );
    } else {
        ctx.accounts.project.set_inner(
            Project::new(
                project_id,
                name,
                admins,
                goal,
                beneficiary,
                *ctx.bumps
                    .get("project")
                    .expect("Failed to derive bump for `project`"),
            )?
        );
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    project_id: u64,
    _name: String, // Anchor barfs if you don't have all ix args
)]
pub struct CreateProject<'info> {
    #[account( 
        init,
        space = Project::SPACE,
        payer = payer,
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}