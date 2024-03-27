use std::cell::Ref;
use std::ops::{Deref, Index};
use solana_program::account_info::{Account, IntoAccountInfo};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::errors::*;

pub trait DevolAccount {
    #[inline(always)]
    fn expected_size() -> usize;

    #[inline(always)]
    fn expected_tag() -> u8;

    #[inline(always)]
    fn expected_version() -> u32;

    #[inline(always)]
    fn account_header<'a>(data: Ref<&mut [u8]>) -> &'a AccountHeader {
        unsafe { &*(data.as_ptr() as *const AccountHeader) }
    }


    #[inline(always)]
    fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<(), u32> {
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        Self::check_size(tag, account_info.data.borrow().len())?;
        let header = Self::account_header(account_info.data.borrow());
        Self::check_tag_and_version(tag, header)?;
        Self::check_root(tag, header, root_addr)?;
        Self::check_program_id(tag, account_info, program_id)?;
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
    fn check_tag_and_version (tag: AccountTag, header: &AccountHeader) -> Result<(), u32> {
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
    fn check_root (tag: AccountTag, header: &AccountHeader, root_addr: &Pubkey) -> Result<(), u32> {
        if header.root != *root_addr {
            Err(error_with_account(tag, ContractError::RootAddress))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn check_program_id (tag: AccountTag, account_info: &AccountInfo, program_id: &Pubkey) -> Result<(), u32> {
        if account_info.owner != program_id {
            Err(error_with_account(tag, ContractError::AccountOwner))
        } else {
            Ok(())
        }
    }

    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    fn from_account_info<'a>(account_info: &'a AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<&'a Self, u32>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id)?;
        let account = unsafe { & *(account_info.data.borrow_mut().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    fn from_account_info_mut<'a>(account_info: &'a AccountInfo, root_addr: &Pubkey, program_id: &Pubkey) -> Result<&'a mut Self, u32>
        where
            Self: Sized,
    {
        Self::check_all(account_info, root_addr, program_id)?;
        if !account_info.is_writable {
            return Err(error_with_account(AccountTag::from_u8(Self::expected_tag()).unwrap(), ContractError::AccountWritableAttribute));
        }
        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    fn from_account(key: &Pubkey, account: &mut impl Account, root_addr: &Pubkey, program_id: &Pubkey) -> Result<Self, u32>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info, root_addr, program_id)?;
        Ok(*account_ref)
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::root::root_account::RootAccount;
    pub const RPC_URL: &str = "https://devnet.helius-rpc.com/?api-key=a4fd5524-2f2d-4713-9acf-aeb92a7e503a";
    pub const INT_SEED: &str = "1012";
    pub const PROGRAM_ID: &str = "2aJHohZdg4oaSuXGQzSDzZC3BJvEoN5JhpBu9GERiroo";
    pub const ADMIN_PUBLIC_KEY: &str = "3PvwxG6kyqKGBwYzWSvkuA8e1GqoChnmDR9WkmjJLPBg";

    #[test]
    fn test_read_root_account() {
        let pubkey = Pubkey::from_str("HrWYxhCJgJ6mpBpkF1yvfdMipHBXA7iciVmGaTTz1rqE").unwrap();
        let client = RpcClient::new(String::from(RPC_URL));
        let mut account_data = client.get_account(&pubkey).unwrap();
        let root_addr = Pubkey::from_str("HrWYxhCJgJ6mpBpkF1yvfdMipHBXA7iciVmGaTTz1rqE").unwrap();
        let program_id = Pubkey::from_str("2aJHohZdg4oaSuXGQzSDzZC3BJvEoN5JhpBu9GERiroo").unwrap();

        assert_eq!(account_data.data.len(), RootAccount::expected_size());

        match RootAccount::from_account(&pubkey, &mut account_data, &root_addr, &program_id) {
            Ok(root_account) => {
                assert!(true, "RootAccount success");
            },
            Err(e) => panic!("Error building RootAccount: {:?}", decode_error_code(e)),
        }
    }

    // #[test]
    // fn test_transform_account_info_mut() {
    //     let data_len = RootAccount::expected_size();
    //     let mut data = vec![0; data_len];
    //     let mut lamports : u64 = 0;
    //
    //     let account_info = AccountInfo {
    //         key: &Pubkey::new_unique(),
    //         is_signer: false,
    //         is_writable: false,
    //         lamports: Rc::new(RefCell::new(&mut lamports)),
    //         data: Rc::new(RefCell::new(&mut *data)),
    //         owner: &Pubkey::new_unique(),
    //         executable: false,
    //         rent_epoch: 0,
    //     };
    //
    //     let result = RootAccount::from_account_info_mut(&account_info);
    //     assert!(result.is_ok());
    //
    //     let root_account = result.unwrap();
    //     assert_eq!(root_account.clients_count, 0);
    // }
}
