use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::state::project::*;
use crate::util::mint_is_supported;

pub fn contribute(
    ctx: Context<Contribute>,
    _project_id: u64,
    amount: u64,
) -> Result<()> {
    // Check to make sure the token is supported
    mint_is_supported(&ctx.accounts.mint.key())?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.payer_token_account.to_account_info(),
                to: ctx.accounts.project_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        amount,
    )?;

    //Increment fields
    ctx.accounts.project.raised += amount;
    ctx.accounts.project.balance += amount;
    ctx.accounts.project.contributors += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    project_id: u64,
    _amount: u64, // Anchor barfs if you don't have all ix args
)]
pub struct Contribute<'info> {
    #[account( 
        mut,
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
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
        token::authority = payer,
    )]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}