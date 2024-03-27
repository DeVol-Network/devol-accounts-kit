use solana_program::account_info::{Account, IntoAccountInfo};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::pubkey::Pubkey;

pub trait DevolAccount {
    #[inline(always)]
    fn expected_size() -> usize;

    #[inline(always)]
    fn check_size(actual_size: usize) -> Result<(), u32> {
        if Self::expected_size() != actual_size {
            Err(123)
        } else {
            Ok(())
        }
    }

    /// Used off-chain to convert raw account data from RPC to a blockchain-utilized account structure.
    fn from_account(key: &Pubkey, account: &mut impl Account) -> Result<Self, u32>
        where
            Self: Sized + Copy
    {
        let account_info = (key, account).into_account_info();
        let account_ref = Self::from_account_info(&account_info)?;
        Ok(*account_ref)
    }


    /// Transforms `AccountInfo` into a reference of `Self` for on-chain use without the intent to modify the data.
    fn from_account_info<'a>(account_info: &'a AccountInfo) -> Result<&'a Self, u32>
        where
            Self: Sized,
    {
        let data_len = account_info.data_len();
        Self::check_size(data_len)?;

        let account = unsafe { & *(account_info.data.borrow_mut().as_ptr() as *const Self) };
        Ok(account)
    }

    /// Transforms `AccountInfo` into a mutable reference of `Self` for on-chain use with the intent to modify the data.
    /// Ensures the account is marked as writable.
    fn from_account_info_mut<'a>(account_info: &'a AccountInfo) -> Result<&'a mut Self, u32>
        where
            Self: Sized,
    {
        let data_len = account_info.data_len();
        Self::check_size(data_len)?;

        let account = unsafe { &mut *(account_info.data.borrow_mut().as_ptr() as *mut Self) };
        Ok(account)
    }

}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::str::FromStr;
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::root::root_account::RootAccount;

    #[test]
    fn test_read_root_account() {
        let pubkey = Pubkey::from_str("HrWYxhCJgJ6mpBpkF1yvfdMipHBXA7iciVmGaTTz1rqE").unwrap();
        let client = RpcClient::new(String::from("https://api.devnet.solana.com/"));
        let mut account_data = client.get_account(&pubkey).unwrap();

        assert_eq!(account_data.data.len(), RootAccount::expected_size());

        match RootAccount::from_account(&pubkey, &mut account_data) {
            Ok(root_account) => {
                assert!(true, "RootAccount success");
            },
            Err(e) => panic!("Error building RootAccount: {:?}", e),
        }
    }

    #[test]
    fn test_transform_account_info_mut() {
        let data_len = RootAccount::expected_size();
        let mut data = vec![0; data_len];
        let mut lamports : u64 = 0;

        let account_info = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: false,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(&mut *data)),
            owner: &Pubkey::new_unique(),
            executable: false,
            rent_epoch: 0,
        };

        let result = RootAccount::from_account_info_mut(&account_info);
        assert!(result.is_ok());

        let root_account = result.unwrap();
        assert_eq!(root_account.clients_count, 0);
    }
}
