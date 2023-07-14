use anchor_lang::prelude::*;

use crate::state::source::*;

pub fn create_source(
    ctx: Context<CreateFundingSource>,
    name: String,
) -> Result<()> {
    ctx.accounts.source.set_inner(
        FundingSource::new(
            name,
            ctx.accounts.payer.key(),
            *ctx.bumps
                .get("source")
                .expect("Failed to derive bump for `source`"),
        )?
    );
    Ok(())
}

#[derive(Accounts)]
pub struct CreateFundingSource<'info> {
    #[account( 
        init,
        space = FundingSource::SPACE,
        payer = payer,
        seeds = [
            FundingSource::SEED_PREFIX.as_bytes(),
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub source: Account<'info, FundingSource>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}