use anchor_lang::prelude::*;

use crate::{error::ProtocolError, util::MAX_NAME_LEN};

#[account]
pub struct FundingSource {
    pub name: String,
    pub authority: Pubkey,
    pub pool_id: u64,
    pub amount: u64,
    pub bump: u8,
}

impl FundingSource {
    pub const SEED_PREFIX: &'static str = "source";

    pub const SPACE: usize = 8 
        + 4 + MAX_NAME_LEN          // String
        + 32                        // Pubkey
        + 8                         // u64
        + 8                         // u64
        + 1                         // u8
        + 8;

    pub fn new(name: String, authority: Pubkey, pool_id: u64, amount: u64, bump: u8) -> Result<Self> {
        if name.as_bytes().len() > MAX_NAME_LEN {
            return Err(ProtocolError::NameTooLong.into());
        }
        Ok(Self {
            name,
            authority,
            pool_id,
            amount,
            bump,
        })
    }
}