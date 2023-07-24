use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
#[derive(Default)]
pub struct Project {
    pub project_id: u64,
    pub name: String,
    pub raised: u8, // Denominated in USDC
    pub goal: u8, // Denominated in USDC
    pub balance: u8, // Denominated in USDC
    pub contributors: u8,
    pub admins: Vec<Pubkey>,
    pub beneficiary: Pubkey,
    pub bump: u8,
    pub status: ProjectStatus,
}

impl Project {
    pub const SEED_PREFIX: &'static str = "fundraiser";

    pub const SPACE: usize = 8 
        + 4                         // u64
        + 4 + MAX_NAME_LEN          // String
        + 32                        // Pubkey
        + 1;                        // u8

    pub fn new(project_id: u64, name: String, admins: Vec<Pubkey>, goal: u8, beneficiary: Pubkey, bump: u8) -> Result<Self> {
        let initial: u8 = 0;
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        Ok(Self {
            project_id,
            name,
            raised: initial,
            goal,
            balance: initial,
            contributors: initial,
            admins,
            beneficiary,
            bump,
            ..Default::default()
        })
    }

    pub fn is_active(&self) -> Result<()> {
        match self.status {
            ProjectStatus::Active => Ok(()),
            ProjectStatus::Deactivated => err!(ProtocolError::DeactivatedProject),
            ProjectStatus::Closed => err!(ProtocolError::ClosedProject),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Active,
    Deactivated,
    Closed,
}
impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Active
    }
}