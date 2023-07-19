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

Copyright 2023 Stockpile,

www.stockpile.pro
www.twitter.com/GoStockpile

DISCLAIMER:
This code is currently unaudited, while reusing and duplication are allowed, please do so at your own risk.
*/

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

pub use instructions::*;

declare_id!("5JGt6XDaPuuUttcsu79Jysd6vqtpDFZqLKi1DiGVhMjv");

#[program]
pub mod stockpile_v2 {
    use super::*;

    pub fn create_project(
        ctx: Context<CreateProject>,
        project_id: u64,
        name: String,
        admins: Vec<Pubkey>,
        beneficiary: Pubkey,
        goal: u8,
    ) -> Result<()> {
        instructions::create_project(ctx, project_id, name, admins, beneficiary, goal)
    }

    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: u64,
        name: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        instructions::create_pool(ctx, pool_id, name, start, end)
    }

    pub fn create_source(
        ctx: Context<CreateSource>,
        name: String,
    ) -> Result<()> {
        instructions::create_source(ctx, name)
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

    pub fn join_pool(
        ctx: Context<JoinPool>,
        _project_id: u64,
        _pool_id: u64,
    ) -> Result<()> {
        instructions::join_pool(ctx, _project_id, _pool_id)
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
}
