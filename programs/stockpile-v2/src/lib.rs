/*


█████╗█████╗█████╗
╚════╝╚════╝╚════╝



███████╗████████╗ ██████╗  ██████╗██╗  ██╗██████╗ ██╗██╗     ███████╗
██╔════╝╚══██╔══╝██╔═══██╗██╔════╝██║ ██╔╝██╔══██╗██║██║     ██╔════╝
███████╗   ██║   ██║   ██║██║     █████╔╝ ██████╔╝██║██║     █████╗
╚════██║   ██║   ██║   ██║██║     ██╔═██╗ ██╔═══╝ ██║██║     ██╔══╝
███████║   ██║   ╚██████╔╝╚██████╗██║  ██╗██║     ██║███████╗███████╗
╚══════╝   ╚═╝    ╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝



█████╗█████╗█████╗
╚════╝╚════╝╚════╝

Copyright 2023 Stockpile Labs,

www.stockpile.so
www.twitter.com/GoStockpile

DISCLAIMER:
This code is currently unaudited, while reusing 
and duplication are allowed, please do so at your
own risk. Please consult the license for more information.
*/

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

pub use instructions::*;
use crate::state::pool::*;

declare_id!("HZR3KsVQAWDRALqtZJWssWXNu9GY9eMt5AQuo2QwSq32");

#[program]
pub mod stockpile_v2 {
    use super::*;

    pub fn create_project(
        ctx: Context<CreateProject>,
        project_id: u64,
        name: String,
        admins: Vec<Pubkey>,
        beneficiary: Pubkey,
        goal: u64,
    ) -> Result<()> {
        instructions::create_project(ctx, project_id, name, admins, beneficiary, goal)
    }

    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: u64,
        name: String,
        start: u64,
        end: u64,
        admins: Vec<Pubkey>,
        access: PoolAccess
    ) -> Result<()> {
        instructions::create_pool(ctx, pool_id, name, start, end, admins, access)
    }

    pub fn create_source(
        ctx: Context<CreateSource>,
        name: String,
        pool_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::create_source(ctx, name, pool_id, amount)
    }

    pub fn create_milestone(
        ctx: Context<CreateMilestone>,
        milestone_id: u64,
        name: String,
        percentage: f64,
    ) -> Result<()> {
        instructions::create_milestone(ctx, milestone_id, name, percentage)
    }

    pub fn contribute(
        ctx: Context<Contribute>,
        _project_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::contribute(ctx, _project_id, amount)
    }

    pub fn contribute_with_vote(
        ctx: Context<ContributeWithVote>,
        _pool_id: u64,
        _project_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::contribute_with_vote(ctx, _pool_id, _project_id, amount)
    }

    pub fn deactivate_project(
        ctx: Context<DeactivateProject>,
    ) -> Result<()> {
        instructions::deactivate_project(ctx)
    }

    pub fn add_project(
        ctx: Context<AddProject>,
        _project_id: u64,
        _pool_id: u64,
    ) -> Result<()> {
        instructions::add_project(ctx, _project_id, _pool_id)
    }

    pub fn join_pool(
        ctx: Context<JoinPool>,
        _project_id: u64,
        _pool_id: u64,
    ) -> Result<()> {
        instructions::join_pool(ctx, _project_id, _pool_id)
    }

    pub fn fund_pool(
        ctx: Context<FundPool>,
        _pool_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::fund_pool(ctx, _pool_id, amount)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn withdraw_all(
        ctx: Context<WithdrawAll>
    ) -> Result<()> {
        instructions::withdraw_all(ctx)
    }

    pub fn close_project(
        ctx: Context<CloseProject>
    ) -> Result<()> {
        instructions::close_project(ctx)
    }

    pub fn close_milestone(
        ctx: Context<CloseMilestone>
    ) -> Result<()> {
        instructions::close_milestone(ctx)
    }

    pub fn claim_payout(
        ctx: Context<ClaimPayout>,
        _project_id: u64,
        _pool_id: u64,
    ) -> Result<()> {
        instructions::claim_payout(ctx, _project_id, _pool_id)
    }

    pub fn withdraw_funds_from_round(
        ctx: Context<WithdrawFromRound>,
        _pool_id: u64
    ) -> Result<()> {
        instructions::withdraw_funds_from_round(ctx, _pool_id)
    }
}
