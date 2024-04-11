use std::error::Error;
use solana_program::account_info::{Account, AccountInfo, IntoAccountInfo};
use solana_program::pubkey::Pubkey;
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
    fn check_id(account_info: &AccountInfo, id: u32) -> Result<(), DvlError> {
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        let read_id = unsafe { *(account_info.data.borrow().as_ptr().add(Self::id_offset()) as *const u32) };
        if read_id != id {
            return Err(DvlError::new_with_account(tag, ContractError::InvalidAccountId));
        }
        Ok(())
    }

    #[inline(always)]
    fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey, id: u32) -> Result<(), DvlError> {
        Self::check_basic(account_info, root_addr, program_id)?;
        Self::check_id(account_info, id)?;
        Ok(())
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    #[inline(always)]
    fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: u32,
    ) -> Result<&'a Self, DvlError>
        where
            Self: Sized,
    {
        let account = Self::from_account_info_basic(account_info,root_addr,program_id)?;
        Self::check_id(account_info, id)?;
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    #[inline(always)]
    fn from_account_info_mut<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: u32,
    ) -> Result<&'a mut Self, DvlError>
        where
            Self: Sized,
    {
        let account = Self::from_account_info_mut_basic(account_info,root_addr,program_id)?;
        Self::check_id(account_info, id)?;
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    #[inline(always)]
    fn from_account(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: u32,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id, id)?;
        Ok(Box::new(*account_ref))
    }
}