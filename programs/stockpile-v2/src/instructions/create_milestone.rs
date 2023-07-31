use anchor_lang::prelude::*;

use crate::state::{
    milestone::*,
    project::*,
};

/// Creates a `Milestone` for a project.
/// These can be added to accompany a fundraiser
pub fn create_milestone(
    ctx: Context<CreateMilestone>,
    milestone_id: u64,
    name: String,
    goal: u64,
) -> Result<()> {
    let payer = &mut ctx.accounts.payer;
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;

    if let Some(payer_key) = project.admins.iter_mut().find(|p| p.key() == *payer_key) {
        ctx.accounts.milestone.set_inner(
            Milestone::new(
                milestone_id,
                name,
                goal,
                project.key(),
                *ctx.bumps
                    .get("milestone")
                    .expect("Failed to derive bump for `project`"),
            )?
        );
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    milestone_id: u64,
    _name: String, // Anchor barfs if you don't have all ix args
)]
pub struct CreateMilestone<'info> {
    #[account( 
        init,
        space = Milestone::SPACE,
        payer = payer,
        seeds = [
            Milestone::SEED_PREFIX.as_bytes(),
            milestone_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub milestone: Account<'info, Milestone>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}