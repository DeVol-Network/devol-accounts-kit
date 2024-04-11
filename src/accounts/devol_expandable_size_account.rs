use std::cell::Ref;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::accounts::devol_account::DevolAccount;
use crate::errors::{AccountTag, ContractError, error_with_account};

pub trait DevolExpandableSizeAccount : DevolAccount {
    #[inline(always)]
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize;

    #[inline(always)]
    fn check_size(tag: AccountTag, account_data: Ref<&mut [u8]>) -> Result<(), u32> {
        let actual_size = account_data.len();
        if actual_size < Self::expected_expanded_size(account_data) {
            Err(error_with_account(tag, ContractError::AccountSize))
        } else {
            Ok(())
        }
    }
}