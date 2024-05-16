use solana_program::account_info::{AccountInfo};
use crate::account_readers::dvl_readable::{DvlParametrable};
use crate::accounts::devol_account::DevolAccount;
use crate::dvl_error::DvlError;
use crate::errors::{AccountTag, ContractError};

pub trait DevolIndexedAccount : DevolAccount {

    /// Returns the offset where an optional ID field is located within the account data.
    /// By default, it assumes that if present, the ID starts immediately after the AccountHeader structure,
    /// i.e., at the 40th byte. This function should be overridden if the ID's position differs.
    #[inline(always)]
    fn id_offset() -> usize { 40 }

    #[inline(always)]
    fn check_id(account_info: &AccountInfo, id: Option<u32>) -> Result<(), DvlError> {
        if let Some(id) = id {
            let tag = AccountTag::from_u8(Self::expected_tag());
            let read_id = unsafe { *(account_info.data.borrow().as_ptr().add(Self::id_offset()) as *const u32) };
            if read_id != id as u32 {
                return Err(DvlError::new_with_account(tag, ContractError::InvalidAccountId));
            }
        }
        Ok(())
    }
}
