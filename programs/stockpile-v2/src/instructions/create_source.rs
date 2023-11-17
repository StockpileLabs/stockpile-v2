use anchor_lang::prelude::*;

use crate::state::source::*;

pub fn create_source(
    ctx: Context<CreateSource>,
    name: String,
    pool_id: u64,
    amount: u64,
) -> Result<()> {
    ctx.accounts.source.set_inner(
        FundingSource::new(
            name,
            ctx.accounts.payer.key(),
            pool_id,
            amount,
            *ctx.bumps
                .get("source")
                .expect("Failed to derive bump for `source`"),
        )?
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    name: String,
    _pool_id: u64,
    amount: u64,
)]
pub struct CreateSource<'info> {
    #[account( 
        init,
        space = FundingSource::SPACE,
        payer = payer,
        seeds = [
            FundingSource::SEED_PREFIX.as_bytes(),
            name.as_ref(),
            _pool_id.to_le_bytes().as_ref(),
            amount.to_le_bytes().as_ref(),
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub source: Account<'info, FundingSource>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}