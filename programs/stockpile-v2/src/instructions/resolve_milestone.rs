use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::error::ProtocolError;
use crate::state::{
    pool::*,
    project::*,
};

/// Setup for milestone resolution. This sends the proportion of project funds
/// put up in the milestone to a Squads multisig for final resolution.
pub fn resolve_milestone(
    ctx: Context<ResolveMilestone>,
    _project_id: u64,
    goal: u64,
    percentage: f64,
) -> Result<()> {
    let payer_key = &mut ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;
    let current_time = Clock::get()?.unix_timestamp as u64;

    mint_is_supported(&ctx.accounts.mint.key())?;

    project.is_active()?;

    if project.admins.contains(&payer_key) {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.project_token_account.to_account_info(),
                    to: ctx.accounts.squad_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            goal * percentage,
        )?;
    } else {
        return Err(ProtocolError::NotAuthorized.into());
    };


    Ok(())
}

#[derive(Accounts)]
#[instruction(
    project_id: u64,
    percentage: f64,
)]
pub struct ResolveMilestone<'info> {
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project.project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    /// CHECK: This is safe because we aren't writing to this account
    pub squad: AccountInfo<'info>,
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
        token::authority = squad,
    )]
    pub squad_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}