use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
#[derive(Default)]
pub struct Project {
    pub project_id: u64,
    pub name: String,
    pub raised: u64, // Denominated in USDC
    pub goal: u64, // Denominated in USDC
    pub balance: u64, // Denominated in USDC
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
        + 4                         // u64
        + 4                         // u64
        + 4                         // u64
        + 1                         // u8
        + 160                       // Vec<Pubkey> (Max 5)
        + 32                        // Pubkey
        + 1                         // u8
        + 4                         // Enum (Singleton)
        + 250;                      // Padding

    pub fn new(project_id: u64, name: String, admins: Vec<Pubkey>, goal: u64, beneficiary: Pubkey, bump: u8) -> Result<Self> {
        let initial: u64 = 0;
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        Ok(Self {
            project_id,
            name,
            raised: initial,
            goal,
            balance: initial,
            contributors: initial as u8,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UpdateField {
    Name(String),
    Goal(u64),
    AddAdmin(Pubkey),
    RemoveAdmin(Pubkey),
    Beneficiary(Pubkey),
    Status(ProjectStatus),
}