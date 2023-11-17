use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::collections::BTreeMap;

use crate::{
    error::ProtocolError,
    util::{to_pubkey, MAX_NAME_LEN, USDC_MINT, USDC_DEVNET_MINT},
};

#[account]
#[derive(Default)]
pub struct Pool {
    pub pool_id: u64,
    pub name: String,
    pub total_funding: u64,
    pub balance: u64,
    pub start: u64,
    pub end: u64,
    pub admins: Vec<Pubkey>,
    pub project_shares: Vec<Participant>,
    pub funders: Vec<FundingTicket>,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";

    pub const SPACE: usize = 8
        + 4                         // u64
        + 4 + MAX_NAME_LEN          // String
        + 4                         // u64
        + 4                         // u64
        + 4                         // u64
        + 160                       // Vec<Pubkey> (Max 5)
        + 32                        // Vec<Participants> (Initial Alloc. for 10 participants w/ 20 votes)
        + 32                        // Vec<FundingTicket> (Initial Alloc. for 5)
        + 4                         // Enum (singleton)
        + 4                         // Enum (singleton)
        + 1                         // u8
        + 2500;                     // Padding

    pub fn new(pool_id: u64, name: String, start: u64, end: u64, admins: Vec<Pubkey>, access: PoolAccess, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > start {
            return Err(ProtocolError::PoolInvalidStart.into());
        }
        Ok(Self {
            pool_id,
            name,
            total_funding: 0,
            balance: 0,
            start,
            end,
            admins,
            project_shares: vec![],
            funders: vec![],
            pool_state: PoolState::PendingStart,
            pool_access: access,
            bump,
        })
    }

    pub fn close_pool(&mut self) -> Result<()> {
        self.is_active()?;
        self.pool_state = PoolState::Closed;
        Ok(())
    }

    pub fn is_active(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.end {
            return Err(ProtocolError::EndDatePassed.into());
        }
        if current_time > self.start {
            self.pool_state = PoolState::Active;
        }
        match self.pool_state {
            PoolState::PendingStart => Err(ProtocolError::PoolNotStarted.into()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProtocolError::ReleasedFunds.into()),
            PoolState::Closed => Err(ProtocolError::PoolClosed.into()),
        }
    }

    pub fn can_fund(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.end {
            return Err(ProtocolError::EndDatePassed.into());
        }
        if current_time > self.start {
            self.pool_state = PoolState::Active;
        }
        match self.pool_state {
            PoolState::PendingStart => Ok(()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProtocolError::ReleasedFunds.into()),
            PoolState::Closed => Err(ProtocolError::PoolClosed.into()),
        }
    }

    pub fn can_withdraw(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.start {
            self.pool_state = PoolState::Active;
        }

        if self.project_shares.len() == 0 && current_time > self.end {
            return Ok(());
        }

        let mut sum: f64 = 0.0;
        for share in &self.project_shares {
            sum += share.share_data.share;
        };

        if sum == 0.0 && current_time > self.end {
            return Ok(());
        }

        match self.pool_state {
            PoolState::PendingStart => Ok(()),
            PoolState::Active => Err(ProtocolError::PoolStillActive.into()),
            PoolState::Distributed => Err(ProtocolError::ReleasedFunds.into()),
            PoolState::Closed => Err(ProtocolError::PoolClosed.into()),
        }
    }

    /// Calculates the total funding amount from current Pyth price data
    pub fn calculate_pool_total_usd(&self, usdc_usd_price: f64) -> Result<f64> {
        let mut pool_total_usd: f64 = 0.0;

        for ticket in &self.funders {
            pool_total_usd +=
                calculate_price_usd(ticket.mint, ticket.amount, usdc_usd_price)?;
        }

        msg!("Pool total USD: {}", pool_total_usd);

        Ok(pool_total_usd)
    }

    pub fn add_participant(&mut self, project_key: Pubkey) -> Result<()> {
        self.project_shares.push(
            Participant::new(
                project_key, 
                PoolShare::new(),
            )
        );

        Ok(())
    }

    /// Updates all shares using the Quadratic Funding algorithm
    pub fn update_shares(
        &mut self,
        pyth_usdc_usd: AccountInfo<'_>,
    ) -> Result<()> {
        // Get the current prices for each mint in USD
        let usdc_usd_price = try_load_price(pyth_usdc_usd)?;

        msg!("USDC/USD Price: {:?}", usdc_usd_price);

        let (vote_count, sum_of_squared_votes_all_projects) = {
            // Block-scope the mutability

            // Set up a `BTreeMap` to use to record each project's squared sum of
            // square roots of votes
            let mut vote_count_mut: BTreeMap<Pubkey, f64> = BTreeMap::new();
            let mut sum_of_squared_votes_all_projects_mut: f64 = 0.0;

            // Iterate through all of the projects
            for project in self.project_shares.iter_mut() {
                // Get the sum of all square roots of each vote
                let total_square_root_votes_usd: f64 = calculate_total_square_root_votes_usd(
                    &project.share_data.votes,
                    usdc_usd_price,
                )?;

                msg!("Sum of square roots of votes: {:?}", total_square_root_votes_usd);

                // Square the sum of all square roots of each vote
                let sum_of_roots_squared = total_square_root_votes_usd.powi(2);

                msg!("Sum of square roots squared: {:?}", sum_of_roots_squared);

                // Add to the vote count `BTreeMap`
                vote_count_mut.insert(project.project_key, sum_of_roots_squared);
                sum_of_squared_votes_all_projects_mut += sum_of_roots_squared;
            }

            (vote_count_mut, sum_of_squared_votes_all_projects_mut)
        };

        // Evaluate each project's distribution from the `vote_count` `HashMap`
        // and update their distribution amount in the `project_shares`
        for project in self.project_shares.iter_mut() {
            let updated_share = match vote_count.get(&project.project_key) {
                Some(vote_count) => vote_count / sum_of_squared_votes_all_projects,
                None => return Err(ProtocolError::AlgorithmFailure.into()),
            };
            project.share_data.share = updated_share;

            msg!("Updated share for {:?} is {:?}", project.project_key, updated_share);
        }

        Ok(())
    }

    /// Issues all payments according to the `project_shares`
    pub fn close_and_open_claim(
        &mut self,
        pyth_usdc_usd: AccountInfo<'_>,
        _accounts: &[AccountInfo<'_>],
    ) -> Result<()> {
        let usdc_usd_price = try_load_price(pyth_usdc_usd)?;
        let _pool_total_usd = self.calculate_pool_total_usd(usdc_usd_price)?;
        
        /*
        for account in _accounts {
            if let Some(participant) = self.project_shares.iter().find(|p| p.project_key == *account.key()) {
                let participant_share_usd = self.total_funding * participant.share_data.share as u64;

                token::transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        token::Transfer {
                            from: ctx.accounts.payer_token_account.to_account_info(),
                            to: ctx.accounts.project_token_account.to_account_info(),
                            authority: payer,
                        },
                    ),
                    amount,
                )?;
            }
        }
        */

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub struct Participant {
    pub project_key: Pubkey,
    pub claimed: bool,
    pub share_data: PoolShare,
}

// Double check this to make sure it works
impl Participant {
    pub fn new(project_key: Pubkey, share_data: PoolShare) -> Self {
        Self {
            project_key: project_key,
            claimed: false,
            share_data: share_data,
        }
    }

    pub fn new_with_vote(project_key: Pubkey, vote: VoteTicket) -> Self {
        let new_share = PoolShare::new_with_vote(vote);
        Self {
            project_key: project_key,
            claimed: false,
            share_data: new_share,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PoolState {
    PendingStart,
    Active,
    Distributed,
    Closed,
}
impl Default for PoolState {
    fn default() -> Self {
        PoolState::PendingStart
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PoolAccess {
    Open,
    Manual,
}
impl Default for PoolAccess {
    fn default() -> Self {
        PoolAccess::Manual
    }
}

fn try_load_price(pyth_account: AccountInfo<'_>) -> Result<f64> {
    match load_price_feed_from_account_info(&pyth_account) {
        Ok(price_feed) => {
            match price_feed.get_price_no_older_than(
                Clock::get()?.unix_timestamp,
                60, // No older than 60 seconds ago
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
    usdc_usd_price: f64,
) -> Result<f64> {
    match mint {
        Some(mint) => {
            if mint.eq(&to_pubkey(USDC_MINT)) {
                Ok(usdc_usd_price * amount as f64)
            } else if mint.eq(&to_pubkey(USDC_DEVNET_MINT)) {
                Ok(usdc_usd_price * amount as f64)
            } else {
                return Err(ProtocolError::MintNotSupported.into());
            }
        }
        None => return Err(ProtocolError::MintNotSupported.into()),
    }
}

fn calculate_total_square_root_votes_usd(
    votes: &Vec<VoteTicket>,
    usdc_usd_price: f64,
) -> Result<f64> {
    let mut total_square_root_votes_usd_mut: f64 = 0.0;

    for vote in votes.iter() {
        let vote_amount_usd =
            calculate_price_usd(vote.mint, vote.amount, usdc_usd_price)?;

        // The square root of the vote's USD amount
        let vote_amount_square_root_usd = vote_amount_usd.sqrt();

        total_square_root_votes_usd_mut += vote_amount_square_root_usd;
    }

    Ok(total_square_root_votes_usd_mut as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn try_update_shares() -> Result<()> {
        let admin = Pubkey::new_unique();
        let vote_tickets = [ 
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 100,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 120,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 190,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 322,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 60,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 10,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 3,
            },
        ];

        let participants = vec![
                Participant {
                    project_key: Pubkey::new_unique(),
                    claimed: false,
                    share_data: PoolShare {
                        share: 0.0,
                        votes: vec![
                            vote_tickets[0].clone(),
                            vote_tickets[2].clone(),
                            vote_tickets[4].clone(),
                            vote_tickets[6].clone()
                        ]
                    }
                },
                Participant {
                    project_key: Pubkey::new_unique(),
                    claimed: false,
                    share_data: PoolShare {
                        share: 0.0,
                        votes: vec![
                            vote_tickets[1].clone(),
                            vote_tickets[3].clone(),
                            vote_tickets[5].clone()
                        ]
                    }
                }
            ];
        
        let mut pool = Pool {
            pool_id: 12345,
            name: "Sample Pool".to_owned(),
            total_funding: 1000,
            balance: 1000,
            start: 0,
            end: 0,
            admins: vec![admin],
            project_shares: participants,
            funders: vec![],
            pool_state: PoolState::Active,
            pool_access: PoolAccess::Open,
            bump: 255,
        };

        fn calculate_total_square_root_votes(
            votes: &Vec<VoteTicket>,
        ) -> Result<f64> {
            let mut total_square_root_votes_usd_mut: f64 = 0.0;

            for vote in votes.iter() {
                let vote_amount_usd =
                    calculate_price_usd(vote.mint, vote.amount, 0.99)?;

                // The square root of the vote's USD amount
                let vote_amount_square_root_usd = vote_amount_usd.sqrt();

                total_square_root_votes_usd_mut += vote_amount_square_root_usd;
            }

            Ok(total_square_root_votes_usd_mut as f64)
        }

        let (vote_count, sum_of_squared_votes_all_projects) = {
            let mut vote_count_mut: BTreeMap<Pubkey, f64> = BTreeMap::new();
            let mut sum_of_squared_votes_all_projects_mut: f64 = 0.0;

            for project in pool.project_shares.iter_mut() {
                let total_square_root_votes_usd: f64 = calculate_total_square_root_votes(
                    &project.share_data.votes,
                )?;

                println!("Sum of square roots of votes for {:?}: {:?}", project.project_key, total_square_root_votes_usd);

                let sum_of_roots_squared = total_square_root_votes_usd.powi(2);

                println!("Sum of square roots squared for {:?}: {:?}", project.project_key, sum_of_roots_squared);

                vote_count_mut.insert(project.project_key, sum_of_roots_squared);
                sum_of_squared_votes_all_projects_mut += sum_of_roots_squared;
            }

            (vote_count_mut, sum_of_squared_votes_all_projects_mut)
        };

        for project in pool.project_shares.iter_mut() {
            let updated_share = match vote_count.get(&project.project_key) {
                Some(vote_count) => vote_count / sum_of_squared_votes_all_projects,
                None => 0.20,
            };
            project.share_data.share = updated_share;

            println!("Updated share for {:?} is {:?}", project.project_key, updated_share);
        }

        Ok(())
    }

    #[test]
    fn try_test_acct_sizing() -> Result<()> {
        let admin = Pubkey::new_unique();
        let vote_tickets = [ 
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 100,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 120,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 190,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 322,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 60,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 10,
            },
            VoteTicket {
                payer: Pubkey::new_unique(),
                mint: Some(to_pubkey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")),
                amount: 3,
            },
        ];

        let participants = vec![
                Participant {
                    project_key: Pubkey::new_unique(),
                    claimed: false,
                    share_data: PoolShare {
                        share: 0.0,
                        votes: vec![
                            vote_tickets[0].clone(),
                            vote_tickets[2].clone(),
                            vote_tickets[4].clone(),
                            vote_tickets[6].clone()
                        ]
                    }
                },
                Participant {
                    project_key: Pubkey::new_unique(),
                    claimed: false,
                    share_data: PoolShare {
                        share: 0.0,
                        votes: vec![
                            vote_tickets[1].clone(),
                            vote_tickets[3].clone(),
                            vote_tickets[5].clone()
                        ]
                    }
                }
            ];
        
        let mut pool = Pool {
            pool_id: 12345,
            name: "Sample Pool".to_owned(),
            total_funding: 1000,
            balance: 1000,
            start: 0,
            end: 0,
            admins: vec![admin],
            project_shares: participants,
            funders: vec![],
            pool_state: PoolState::Active,
            pool_access: PoolAccess::Open,
            bump: 255,
        };

        let mut pool_data = pool.clone();

        let participant = Participant {
                project_key: Pubkey::new_unique(),
                claimed: false,
                share_data: PoolShare {
                share: 0.0,
                votes: vec![]
                }
        };

        pool_data.project_shares.push(participant);

        let new_acct_data = pool_data.try_to_vec()?.len();
        let old_acct_data = pool.try_to_vec()?.len();

        println!("Old Length: {:?} --- New Length: {:?}", old_acct_data, new_acct_data);

        Ok(())
    }
}