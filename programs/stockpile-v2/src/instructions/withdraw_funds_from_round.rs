use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use crate::error::ProtocolError;

use crate::state::pool::*;

/// Withdraws all of the USDC from the round
/// and sends it back to the payer. Requires that the payer
/// be a fundraiser admin. This can only be called before a round begins,
/// or after a round is closed (if there is no participants).
pub fn withdraw_funds_from_round(ctx: Context<WithdrawFromRound>, pool_id: u64) -> Result<()> {
    let payer_key = ctx.accounts.payer.key();
    let pool = &mut ctx.accounts.pool;

    // Check to make sure caller is an admin
    require!(pool.admins.contains(&payer_key), ProtocolError::NotAuthorized);

    pool.can_withdraw()?;

    let bump = pool.bump.to_le_bytes();
    let id_ref = pool_id.to_le_bytes();

    let seeds = vec![Pool::SEED_PREFIX.as_bytes(), id_ref.as_ref(), &bump];
    let signer_seeds = vec![seeds.as_slice()];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.payer_token_account.to_account_info(),
                authority: pool.to_account_info(),
            },
            signer_seeds.as_slice(),
        ),
        pool.total_funding,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _pool_id: u64
)]
pub struct WithdrawFromRound<'info> {
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            _pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,
    pub mint: Account<'info, token::Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = pool,
    )]
    pub pool_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
    )]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    #[account(
        mut,
        constraint = pool.admins.contains(&payer.key())
    )]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}