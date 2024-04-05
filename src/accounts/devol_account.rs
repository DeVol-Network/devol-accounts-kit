use std::cell::Ref;
use std::error::Error;
use solana_program::account_info::{Account, IntoAccountInfo};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::errors::*;

pub trait DevolAccount {
    fn expected_size() -> usize;

    fn expected_tag() -> u8;

    fn expected_version() -> u32;

    /// Returns the offset where an optional ID field is located within the account data.
    /// By default, it assumes that if present, the ID starts immediately after the AccountHeader structure,
    /// i.e., at the 40th byte. This function should be overridden if the ID's position differs.
    #[inline(always)]
    fn id_offset_if_available() -> usize { 40 }


    #[inline(always)]
    fn account_header<'a>(data: Ref<&mut [u8]>) -> &'a AccountHeader {
        unsafe { &*(data.as_ptr() as *const AccountHeader) }
    }

    #[inline(always)]
    fn check_id(tag: AccountTag, account_info: &AccountInfo, id: u32) -> Result<(), u32> {
        let read_id = unsafe { *(account_info.data.borrow().as_ptr().add(Self::id_offset_if_available()) as *const u32) };
        if read_id != id {
            return Err(error_with_account(tag, ContractError::InvalidAccountId));
        }
        Ok(())
    }


    #[inline(always)]
    fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey, id: Option<u32>) -> Result<(), u32> {
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        Self::check_size(tag, account_info.data.borrow().len())?;
        let header = Self::account_header(account_info.data.borrow());
        Self::check_tag_and_version(tag, header)?;
        Self::check_root(tag, header, root_addr)?;
        Self::check_program_id(tag, account_info, program_id)?;
        if let Some(id) = id {
            Self::check_id(tag, account_info, id)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn check_size(tag: AccountTag, actual_size: usize) -> Result<(), u32> {
        if Self::expected_size() != actual_size {
            Err(error_with_account(tag, ContractError::AccountSize))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_tag_and_version(tag: AccountTag, header: &AccountHeader) -> Result<(), u32> {
        if header.tag != Self::expected_tag() as u32 {
            Err(error_with_account(tag, ContractError::AccountTag))
        } else if header.version > Self::expected_version() {
            Err(error_with_account(tag, ContractError::AccountVersionTooHigh))
        } else if header.version < Self::expected_version() {
            Err(error_with_account(tag, ContractError::AccountVersionTooLow))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_root(tag: AccountTag, header: &AccountHeader, root_addr: &Pubkey) -> Result<(), u32> {
        if header.root != *root_addr {
            Err(error_with_account(tag, ContractError::RootAddress))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_program_id(tag: AccountTag, account_info: &AccountInfo, program_id: &Pubkey) -> Result<(), u32> {
        if account_info.owner != program_id {
            Err(error_with_account(tag, ContractError::AccountOwner))
        } else {
            Ok(())
        }
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    fn from_account_info<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: Option<u32>,
    ) -> Result<&'a Self, u32>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, id)?;
        let account = unsafe { &*(account_info.data.borrow().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    fn from_account_info_mut<'a>(
        account_info: &'a AccountInfo,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: Option<u32>,
    ) -> Result<&'a mut Self, u32>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id, id)?;
        if !account_info.is_writable {
            return Err(error_with_account(AccountTag::from_u8(Self::expected_tag()).unwrap(), ContractError::AccountWritableAttribute));
        }
        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    fn from_account(
        key: &Pubkey,
        account: &mut impl Account,
        root_addr: &Pubkey,
        program_id: &Pubkey,
        id: Option<u32>,
    ) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id, id)
            .map_err(ReadAccountError::from)?;
        Ok(*account_ref)
    }
}

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

        match RootAccount::from_account(&root_addr, &mut account_data, &root_addr, &program_id, None) {
            Ok(root_account) => {
                assert!(true, "RootAccount success");
                assert_eq!(root_account.header.tag, ROOT_ACCOUNT_TAG as u32);
            }
            Err(e) => panic!("Error building RootAccount: {:?}", e),
        }
    }
}
