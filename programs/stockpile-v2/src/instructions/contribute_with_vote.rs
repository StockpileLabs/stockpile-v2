use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

use solana_gateway::Gateway;

use crate::error::ProtocolError;
use crate::state::{
    pool::*,
    project::*,
};
use crate::util::{ 
    USDC_USD_PRICE_FEED_ID, 
    set_and_maybe_realloc,
    mint_is_supported,
    to_pubkey
};

// Makes a contribution to a fundraiser that is
// currently participating in a pool. Requires
// that the fundraiser has invoked the "join_pool"
// instruction, and is actively participating. Also
// requires the payer have a valid Civic pass.
pub fn contribute_with_vote(
    ctx: Context<ContributeWithVote>,
    _pool_id: u64,
    _project_id: u64,
    amount: u64,
) -> Result<()> {
    // Define verification options
    let gatekeeper_network = ctx.accounts.gatekeeper_network.key();
    let payer = ctx.accounts.payer.key();

    // Check to make sure the token is supported
    mint_is_supported(&ctx.accounts.mint.key())?;

    // Check to make sure the pool is not closed
    ctx.accounts.pool.is_active()?;

    // Perform Civic pass verification
    Gateway::verify_gateway_token_account_info(
        &ctx.accounts.gateway_token_account.to_account_info(), 
        &ctx.accounts.payer.key(), 
        &gatekeeper_network,
        None,
    ).map_err(|_e| {
        msg!("Gateway token verification failed.");
        ProtocolError::CivicFailure
    });

    // Add the project to the shares, if it doesn't exist
    let project_key = ctx.accounts.project.key();
    let mut pool_data = ctx.accounts.pool.clone().into_inner();

    // Iterate through the Participants, and 
    // check if the project exists in the pool
    // If not: break function and return error
    if let Some(participant) = pool_data.project_shares.iter_mut().find(|p| p.project_key == project_key) {
        let vote_ticket = VoteTicket::new(
            payer, 
            Some(ctx.accounts.mint.key()), 
            amount, 
        );

    if let Some(payer_vote_ticket) = participant.share_data.votes.iter_mut().find(|t| t.payer == payer) {
        payer_vote_ticket.amount += amount;
    } else {
        participant.share_data.votes.push(vote_ticket);
    }
        set_and_maybe_realloc(
            &mut ctx.accounts.pool, 
            &pool_data, 
            ctx.accounts.payer.to_account_info(), 
            ctx.accounts.system_program.to_account_info()
        )?;

        ctx.accounts.pool.set_inner(pool_data);
    } else {
        return Err(ProtocolError::NotInPool.into());
    }

    // Transfer the vote to the project
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

    // Increment fields
    ctx.accounts.project.raised += amount;
    ctx.accounts.project.balance += amount;
    ctx.accounts.project.contributors += 1;

    // Update the QF algorithm
    ctx.accounts.pool.update_shares(
        ctx.accounts.pyth_usdc_usd.to_account_info(),
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
    project_id: u64,
    _amount: u64, // Anchor barfs if you don't have all ix args
)]
pub struct ContributeWithVote<'info> {
    /// CHECK: Pyth will check this
    #[account(
        address = to_pubkey(USDC_USD_PRICE_FEED_ID)
            @ ProtocolError::PythAccountInvalid
    )]
    pub pyth_usdc_usd: UncheckedAccount<'info>,
    #[account( 
        mut,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,
    #[account( 
        seeds = [
            Project::SEED_PREFIX.as_bytes(),
            project_id.to_le_bytes().as_ref(),
        ],
        bump = project.bump,
    )]
    pub project: Box<Account<'info, Project>>,
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
    /// CHECK: This is not unsafe because this account isn't written to
    pub gateway_token_account: AccountInfo<'info>,
    /// CHECK: This is not unsafe because this account isn't written to
    pub gatekeeper_network: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}