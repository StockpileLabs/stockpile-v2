use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::error::ProtocolError;

use crate::state::project::*;

/// Withdraws all of the USDC from the vault
/// and sends to the beneficiary. Requires that the payer
/// be a fundraiser admin.
pub fn withdraw_all(ctx: Context<WithdrawAll>) -> Result<()> {
    let payer_key = ctx.accounts.payer.key();
    let project = &mut ctx.accounts.project;

    // Check to make sure caller is an admin
    require!(project.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.project_token_account.to_account_info(),
                to: ctx.accounts.beneficiary_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        project.balance.into(),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawAll<'info> {
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project.project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,
    #[account(mut, constraint = beneficiary.key() == project.beneficiary)]
    pub beneficiary: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        token::mint = mint,
        token::authority = beneficiary,
    )]
    pub beneficiary_token_account: Account<'info, token::TokenAccount>,
    pub mint: Account<'info, token::Mint>,
    #[account(
        init_if_needed,
        payer = payer,
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