use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
pub struct FundingSource {
    pub name: String,
    pub authority: Pubkey,
    pub bump: u8,
}

impl FundingSource {
    pub const SEED_PREFIX: &'static str = "source";

    pub const SPACE: usize = 8 
        + 4 + MAX_NAME_LEN          // String
        + 32                        // Pubkey
        + 1;                        // u8

    pub fn new(name: String, authority: Pubkey, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        Ok(Self {
            name,
            authority,
            bump,
        })
    }
}