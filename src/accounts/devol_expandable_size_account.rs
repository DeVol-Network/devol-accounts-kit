use std::cell::Ref;
use crate::errors::{AccountTag, ContractError, error_with_account};

pub trait DevolExpandableSizeAccount {
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize;

    #[inline(always)]
    fn check_expanded_size(tag: AccountTag, account_data: Ref<&mut [u8]>) -> Result<(), u32> {
        let actual_size = account_data.len();
        if actual_size < Self::expected_expanded_size(account_data) {
            Err(error_with_account(tag, ContractError::AccountSize))
        } else {
            Ok(())
        }
    }
}