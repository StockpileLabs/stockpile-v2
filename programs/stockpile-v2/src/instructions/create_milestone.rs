use anchor_lang::prelude::*;

use crate::{state::{
    milestone::*,
    project::*,
}, error::ProtocolError};

/// Creates a `Milestone` for a project.
/// These can be added to accompany a fundraiser
pub fn create_milestone(
    ctx: Context<CreateMilestone>,
    milestone_id: u64,
    name: String,
    percentage: f64,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;

    if project.admins.iter().any(|admin| admin == payer_key) {
        ctx.accounts.milestone.set_inner(
            Milestone::new(
                milestone_id,
                name,
                percentage,
                project.key(),
                *ctx.bumps
                    .get("milestone")
                    .expect("Failed to derive bump for `project`"),
            )?
        );
    } else {
        return Err(ProtocolError::NotAuthorized.into());
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    milestone_id: u64,
    name: String, // Anchor barfs if you don't have all ix args
)]
pub struct CreateMilestone<'info> {
    #[account( 
        init,
        space = Milestone::SPACE,
        payer = payer,
        seeds = [
            Milestone::SEED_PREFIX.as_bytes(),
            name.as_ref(),
            milestone_id.to_le_bytes().as_ref(),
            project.key().as_ref(),
        ],
        bump,
    )]
    pub milestone: Account<'info, Milestone>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}