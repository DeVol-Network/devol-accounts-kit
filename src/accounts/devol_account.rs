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

    fn try_from_slice(data: &[u8]) -> Result<Self, u32>
        where
            Self: Sized + Default,
    {
        Self::check_size(data.len())?;
        let mut instance = Self::default();
        let instance_bytes = unsafe {
            std::slice::from_raw_parts_mut(&mut instance as *mut _ as *mut u8, Self::expected_size())
        };
        instance_bytes.copy_from_slice(data);
        Ok(instance)
    }

    fn transform_account_info_mut<'a>(account_info: &'a AccountInfo) -> Result<&'a mut Self, u32>
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
    use std::str::FromStr;
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_client::rpc_client::RpcClient;
    use crate::accounts::root::root_account::RootAccount;

    #[test]
    fn test_read_root_account() {
        let pubkey = Pubkey::from_str("HrWYxhCJgJ6mpBpkF1yvfdMipHBXA7iciVmGaTTz1rqE").unwrap();
        let client = RpcClient::new(String::from("https://api.devnet.solana.com/"));
        let account_data = client.get_account_data(&pubkey).expect("Unable to read account data");

        assert_eq!(account_data.len(), RootAccount::expected_size());

        match RootAccount::try_from_slice(&account_data) {
            Ok(root_account) => {
                assert!(true, "RootAccount success");
            },
            Err(e) => panic!("Error building RootAccount: {:?}", e),
        }
    }
}
