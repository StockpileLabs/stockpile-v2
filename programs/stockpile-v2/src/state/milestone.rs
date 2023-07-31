use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::{MAX_NAME_LEN, MAX_ADMIN_LEN}};

#[account]
#[derive(Default)]
pub struct Milestone {
    pub milestone_id: u64,
    pub name: String,
    pub local_raised: u64, // Denominated in USDC
    pub local_goal: u64, // Denominated in USDC
    pub local_balance: u64, // Denominated in USDC
    pub contributors: u8,
    pub associated_project: Pubkey,
    pub bump: u8,
    pub status: MilestoneStatus,
}

impl Milestone {
    pub const SEED_PREFIX: &'static str = "milestone";

    pub const SPACE: usize = 8 
        + 4                         // u64
        + 4 + MAX_NAME_LEN          // String
        + 32                        // Pubkey
        + 8                         // u64
        + 8                         // u64
        + 8                         // u64
        + 1                         // u8
        + 1                         // u8
        + 1 + 42;                   // Enum

    pub fn new(milestone_id: u64, name: String, goal: u64, associated_project: Pubkey, bump: u8) -> Result<Self> {
        let initial: u64 = 0;
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        Ok(Self {
            milestone_id,
            name,
            local_raised: initial,
            local_goal: goal,
            local_balance: initial,
            contributors: initial as u8,
            associated_project,
            bump,
            ..Default::default()
        })
    }

    pub fn deactivate(&mut self) -> Result<()> {
        self.is_active()?;
        self.status = MilestoneStatus::Deactivated;
        Ok(())
    }

    pub fn reactivate(&mut self) -> Result<()> {
        self.is_active()?;
        self.status = MilestoneStatus::Active;
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.is_active()?;
        self.status = MilestoneStatus::Closed;
        Ok(())
    }

    pub fn is_active(&self) -> Result<()> {
        match self.status {
            MilestoneStatus::Active => Ok(()),
            MilestoneStatus::Deactivated => err!(ProtocolError::DeactivatedProject),
            MilestoneStatus::Closed => err!(ProtocolError::ClosedProject),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MilestoneStatus {
    Active,
    Deactivated,
    Closed,
}
impl Default for MilestoneStatus {
    fn default() -> Self {
        MilestoneStatus::Active
    }
}