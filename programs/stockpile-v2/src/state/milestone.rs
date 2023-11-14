use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
#[derive(Default)]
pub struct Milestone {
    pub milestone_id: u64,
    pub name: String,
    pub percentage: f64,
    pub close: u64,
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
        + 1 + 42                    // Enum
        + 250;                      // Padding

    pub fn new(milestone_id: u64, name: String, percentage: f64, associated_project: Pubkey, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }

        Ok(Self {
            milestone_id,
            name,
            percentage,
            close: 0,
            associated_project,
            bump,
            ..Default::default()
        })
    }

    pub fn reconcile(&mut self) -> Result<()> {
        self.status = MilestoneStatus::Reconciling;

        let current_time = Clock::get()?.unix_timestamp as u64;

        self.close = current_time + 259200;

        Ok(())
    }

    pub fn is_active(&self) -> Result<()> {
        match self.status {
            MilestoneStatus::Active => Ok(()),
            MilestoneStatus::Reconciling => err!(ProtocolError::MilestoneIsReconciling),
            MilestoneStatus::Closed => err!(ProtocolError::ClosedMilestone),
        }
    }

    pub fn is_reconciling(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;

        if current_time < self.close {
            return Err(ProtocolError::MilestoneIsReconciling.into());
        }

        match self.status {
            MilestoneStatus::Active => err!(ProtocolError::OpenMilestone),
            MilestoneStatus::Reconciling => Ok(()),
            MilestoneStatus::Closed => err!(ProtocolError::ClosedMilestone),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MilestoneStatus {
    Active,
    Reconciling,
    Closed,
}
impl Default for MilestoneStatus {
    fn default() -> Self {
        MilestoneStatus::Active
    }
}