use anchor_lang::prelude::*;
use anchor_lang::system_program;
use std::str::FromStr;

use crate::error::ProtocolError;

pub const MAX_NAME_LEN: usize = 50;

pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub const SOL_USD_PRICE_FEED_ID: &str = "ALP8SdU9oARYVLgLR7LrqMNCYBnhtnQz1cj6bwgwQmgj";
pub const USDC_USD_PRICE_FEED_ID: &str = "8GWTTbNiXdmyZREXbjsZBmCRuzdPrW55dnZGDkTRjWvb";

pub const SUPPORTED_SPL_MINTS: [&'static str; 1] = [USDC_MINT];

pub fn to_pubkey(string: &str) -> Pubkey {
    Pubkey::from_str(&string).expect("Error parsing public key from string.")
}

pub fn mint_is_supported(mint_pubkey: &Pubkey) -> Result<()> {
    for suppported_mint in SUPPORTED_SPL_MINTS {
        if to_pubkey(suppported_mint).eq(mint_pubkey) {
            return Ok(());
        }
    }
    Err(ProtocolError::MintNotSupported.into())
}

pub fn set_and_maybe_realloc<'info, T>(
    account: &mut Account<'info, T>,
    new_data: T,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()>
where
    T: AccountDeserialize
        + AccountSerialize
        + borsh::BorshDeserialize
        + borsh::BorshSerialize
        + Clone + anchor_lang::Owner,
{
    let account_info = account.to_account_info();

    // See if it needs to be reallocated
    let new_account_size = (new_data.try_to_vec()?).len();
    if new_account_size > account_info.data_len() {
        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        // Perform transfer of additional rent
        system_program::transfer(
            CpiContext::new(
                system_program,
                system_program::Transfer {
                    from: payer,
                    to: account_info.clone(),
                },
            ),
            additional_rent_to_fund,
        )?;

        // Serialize new data
        account_info.realloc(new_account_size, false)?;
    }
    account.set_inner(new_data);
    Ok(())
}