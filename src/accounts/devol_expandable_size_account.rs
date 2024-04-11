use std::cell::Ref;
use crate::dvl_error::DvlError;
use crate::errors::{AccountTag, ContractError};

pub trait DevolExpandableSizeAccount {
    fn expected_expanded_size(account_data: Ref<&mut [u8]>) -> usize;

    #[inline(always)]
    fn check_expanded_size(tag: AccountTag, account_data: Ref<&mut [u8]>) -> Result<(), DvlError> {
        let actual_size = account_data.len();
        if actual_size < Self::expected_expanded_size(account_data) {
            Err(DvlError::new_with_account(tag, ContractError::AccountSize))
        } else {
            Ok(())
        }
    }
}