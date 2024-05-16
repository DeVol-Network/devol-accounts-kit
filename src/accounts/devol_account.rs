use std::cell::Ref;
use std::error::Error;
use solana_program::account_info::{Account, IntoAccountInfo};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::account_readers::dvl_readable::{DvlParametrable};
use crate::accounts::account_header::AccountHeader;
use crate::dvl_error::DvlError;
use crate::errors::*;

pub trait DevolAccount: DvlParametrable{
    fn expected_size() -> usize;

    fn expected_tag() -> u8;

    fn expected_version() -> u32;

    #[inline(always)]
    fn check_additional<'a>(_account_info: &AccountInfo, _params: &Self::DvlReadParams<'a>) -> Result<(), DvlError>{
        Ok(())
    }

    #[inline(always)]
    fn account_header<'a>(data: Ref<&mut [u8]>) -> &'a AccountHeader {
        unsafe { &*(data.as_ptr() as *const AccountHeader) }
    }

    #[inline(always)]
    fn check_basic(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<(), DvlError> {
        let tag = AccountTag::from_u8(Self::expected_tag());
        Self::check_size(tag, account_info.data.borrow())?;
        let header = Self::account_header(account_info.data.borrow());
        Self::check_tag_and_version(tag, header)?;
        Self::check_root(tag, header, root_addr)?;
        Self::check_program_id(tag, account_info, program_id)?;
        Ok(())
    }

    #[inline(always)]
    fn check_size(tag: AccountTag, account_data: Ref<&mut [u8]>) -> Result<(), DvlError> {
        let actual_size= account_data.len();
        if actual_size < Self::expected_size() {
            Err(DvlError::new_with_account(tag, ContractError::AccountSize))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_tag_and_version(tag: AccountTag, header: &AccountHeader) -> Result<(), DvlError> {
        if header.tag != Self::expected_tag() as u32 {
            Err(DvlError::new_with_account(tag, ContractError::WrongAccountTag))
        } else if header.version > Self::expected_version() {
            Err(DvlError::new_with_account(tag, ContractError::AccountVersionTooHigh))
        } else if header.version < Self::expected_version() {
            Err(DvlError::new_with_account(tag, ContractError::AccountVersionTooLow))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_root(tag: AccountTag, header: &AccountHeader, root_addr: &Pubkey) -> Result<(), DvlError> {
        if header.root != *root_addr {
            Err(DvlError::new_with_account(tag, ContractError::RootAddress))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_program_id(tag: AccountTag, account_info: &AccountInfo, program_id: &Pubkey) -> Result<(), DvlError> {
        if account_info.owner != program_id {
            Err(DvlError::new_with_account(tag, ContractError::AccountOwner))
        } else {
            Ok(())
        }
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    #[inline(always)]
    fn from_account_info_basic<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<&'a Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_basic(account_info, root_addr, program_id)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    #[inline(always)]
    fn from_account_info_mut_basic<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<&'a mut Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_basic(account_info, root_addr, program_id)?;
        if !account_info.is_writable {
            return Err(DvlError::new_with_account(AccountTag::from_u8(Self::expected_tag()), ContractError::AccountWritableAttribute));
        }
        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    #[inline(always)]
    fn from_account_basic(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info_basic(&account_info, root_addr, program_id)?;
        Ok(Box::new(*account_ref))
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    #[inline(always)]
    fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        params: &Self::DvlReadParams<'a>,
    ) -> Result<&'a Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_basic(account_info, root_addr, program_id)?;
        Self::check_additional(&account_info, params)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    #[inline(always)]
    fn from_account_info_mut<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        params: &Self::DvlReadParams<'a>,
    ) -> Result<&'a mut Self, DvlError>
        where
            Self: Sized,
    {
        Self::check_basic(account_info, root_addr, program_id)?;
        Self::check_additional(&account_info, params)?;
        if !account_info.is_writable {
            return Err(DvlError::new_with_account(AccountTag::from_u8(Self::expected_tag()), ContractError::AccountWritableAttribute));
        }
        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    #[inline(always)]
    fn from_account<'a>(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        params: &Self::DvlReadParams<'a>,
    ) -> Result<Box<Self>, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info_basic(&account_info, root_addr, program_id)?;
        println!("from_account basic");
        Self::check_additional(&account_info, params)?;
        Ok(Box::new(*account_ref))
    }
}

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::root::root_account::{ROOT_ACCOUNT_TAG, RootAccount};
    use crate::constants::test_constants::{PROGRAM_ID, ROOT_ADDRESS, RPC_URL};

    #[test]
    fn test_read_root_account() {
        let root_addr = Pubkey::from_str(ROOT_ADDRESS).unwrap();
        let client = RpcClient::new(String::from(RPC_URL));
        let mut account_data = client.get_account(&root_addr).unwrap();
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

        assert_eq!(account_data.data.len(), RootAccount::expected_size());
        match RootAccount::from_account(&root_addr, &mut account_data, &root_addr, &program_id, &()) {
            Ok(root_account) => {
                assert!(true, "RootAccount success");
                assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
            }
            Err(e) => panic!("Error building RootAccount: {:?}", e),
        }
    }
}
