use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::collections::HashMap;

use crate::{
    error::ProtocolError,
    util::{to_pubkey, MAX_NAME_LEN, USDC_MINT},
};

#[account]
pub struct Pool {
    pub pool_id: u64,
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub project_shares: HashMap<Pubkey, PoolShare>,
    pub funders: Vec<FundingTicket>,
    pub pool_state: PoolState,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";

    pub const SPACE: usize = 8
        + 4                         // u64
        + 4 + MAX_NAME_LEN          // String
        + 4                         // u64
        + 4                         // u64
        + 4                         // HashMap (empty)
        + 4                         // Vec (empty)
        + 1                         // Enum (singleton)
        + 1; // u8

    pub fn new(pool_id: u64, name: String, start: u64, end: u64, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time < start {
            return Err(ProtocolError::PoolInvalidStart.into());
        }
        Ok(Self {
            pool_id,
            name,
            start,
            end,
            project_shares: HashMap::new(),
            funders: vec![],
            pool_state: PoolState::PendingStart,
            bump,
        })
    }

    pub fn close_pool(&mut self) -> Result<()> {
        self.is_active()?;
        self.pool_state = PoolState::Closed;
        Ok(())
    }

    pub fn is_active(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.end {
            return Err(ProtocolError::EndDatePassed.into());
        }
        match self.pool_state {
            PoolState::PendingStart => Err(ProtocolError::PoolNotStarted.into()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProtocolError::ReleasedFunds.into()),
            PoolState::Closed => Err(ProtocolError::PoolClosed.into()),
        }
    }

    /// Calculates the total funding amount from current Pyth price data
    pub fn calculate_pool_total_usd(&self, sol_usd_price: f64, usdc_usd_price: f64) -> Result<f64> {
        let mut pool_total_usd: f64 = 0.0;

        for ticket in &self.funders {
            pool_total_usd +=
                calculate_price_usd(ticket.mint, ticket.amount, sol_usd_price, usdc_usd_price)?;
        }

        Ok(pool_total_usd)
    }

    /// Updates all shares using the Quadratic Funding algorithm
    pub fn update_shares(
        &mut self,
        pyth_sol_usd: AccountInfo<'_>,
        pyth_usdc_usd: AccountInfo<'_>,
    ) -> Result<()> {
        // Get the current prices for each mint in USD
        let sol_usd_price = try_load_price(pyth_sol_usd)?;
        let usdc_usd_price = try_load_price(pyth_usdc_usd)?;

        let (vote_count, sum_of_squared_votes_all_projects) = {
            // Block-scope the mutability

            // Set up a `HashMap` to use to record each project's squared sum of
            // square roots of votes
            let mut vote_count_mut: HashMap<Pubkey, f64> = HashMap::new();
            let mut sum_of_squared_votes_all_projects_mut: f64 = 0.0;

            // Iterate through all of the projects
            for project in self.project_shares.iter() {
                // Get the sum of all square roots of each vote
                let total_square_root_votes_usd: f64 = calculate_total_square_root_votes_usd(
                    &project.1.votes,
                    sol_usd_price,
                    usdc_usd_price,
                )?;

                // Square the sum of all square roots of each vote
                let sum_of_roots_squared = total_square_root_votes_usd.powi(2);

                // Add to the vote count `HashMap`
                vote_count_mut.insert(*project.0, sum_of_roots_squared);
                sum_of_squared_votes_all_projects_mut += sum_of_roots_squared;
            }

            (vote_count_mut, sum_of_squared_votes_all_projects_mut)
        };

        // Evaluate each project's distribution from the `vote_count` `HashMap`
        // and update their distribution amount in the `project_shares`
        for project in self.project_shares.iter_mut() {
            let updated_share = match vote_count.get(project.0) {
                Some(vote_count) => vote_count / sum_of_squared_votes_all_projects,
                None => return Err(ProtocolError::AlgorithmFailure.into()),
            };
            project.1.share = updated_share;
        }

        Ok(())
    }

    /// Issues all payments according to the `project_shares`
    pub fn close_and_issue_payments(
        &mut self,
        pyth_sol_usd: AccountInfo<'_>,
        pyth_usdc_usd: AccountInfo<'_>,
        _accounts: &[AccountInfo<'_>],
    ) -> Result<()> {
        let sol_usd_price = try_load_price(pyth_sol_usd)?;
        let usdc_usd_price = try_load_price(pyth_usdc_usd)?;
        let _pool_total_usd = self.calculate_pool_total_usd(sol_usd_price, usdc_usd_price)?;
        // TODO: Leverage "additional accounts" to match up `Project` addresses
        // and pay every project out
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FundingTicket {
    pub source: Pubkey,
    pub mint: Option<Pubkey>,
    pub amount: u64,
}

impl FundingTicket {
    pub fn new(source: Pubkey, mint: Option<Pubkey>, amount: u64) -> Self {
        Self {
            source,
            mint,
            amount,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolShare {
    pub share: f64,
    pub votes: Vec<VoteTicket>,
}

impl PoolShare {
    pub fn new() -> Self {
        Self {
            share: 0.0,
            votes: vec![],
        }
    }

    pub fn new_with_vote(vote: VoteTicket) -> Self {
        Self {
            share: 0.0,
            votes: vec![vote],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoteTicket {
    pub payer: Pubkey,
    pub mint: Option<Pubkey>,
    pub amount: u64,
}

impl VoteTicket {
    pub fn new(payer: Pubkey, mint: Option<Pubkey>, amount: u64) -> Self {
        Self {
            payer,
            mint,
            amount,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PoolState {
    PendingStart,
    Active,
    Distributed,
    Closed,
}

fn try_load_price(pyth_account: AccountInfo<'_>) -> Result<f64> {
    match load_price_feed_from_account_info(&pyth_account) {
        Ok(price_feed) => {
            match price_feed.get_price_no_older_than(
                Clock::get()?.unix_timestamp,
                20, // No older than 20 seconds ago
            ) {
                Some(price) => Ok(price.price as f64),
                None => Err(ProtocolError::PythPriceFeedPriceFailed.into()),
            }
        }
        Err(_) => Err(ProtocolError::PythPriceFeedLoadFailed.into()),
    }
}

fn calculate_price_usd(
    mint: Option<Pubkey>,
    amount: u64,
    sol_usd_price: f64,
    usdc_usd_price: f64,
) -> Result<f64> {
    match mint {
        Some(mint) => {
            if mint.eq(&to_pubkey(USDC_MINT)) {
                Ok(usdc_usd_price * amount as f64)
            } else {
                return Err(ProtocolError::MintNotSupported.into());
            }
        }
        None => Ok(sol_usd_price * amount as f64),
    }
}

fn calculate_total_square_root_votes_usd(
    votes: &Vec<VoteTicket>,
    sol_usd_price: f64,
    usdc_usd_price: f64,
) -> Result<f64> {
    let mut total_square_root_votes_usd_mut: f64 = 0.0;

    for vote in votes.iter() {
        let vote_amount_usd =
            calculate_price_usd(vote.mint, vote.amount, sol_usd_price, usdc_usd_price)?;

        // The square root of the vote's USD amount
        let vote_amount_square_root_usd = vote_amount_usd.sqrt();

        total_square_root_votes_usd_mut += vote_amount_square_root_usd;
    }

    Ok(total_square_root_votes_usd_mut as f64)
}