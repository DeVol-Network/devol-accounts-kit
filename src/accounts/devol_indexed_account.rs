use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::worker::pools_log::pools_log_account::{POOLS_LOG_ACCOUNT_SIZE, POOLS_LOG_ACCOUNT_TAG, POOLS_LOG_ACCOUNT_VERSION};
use crate::errors::AccountTag;

pub trait DevolIndexedAccount : DevolAccount {
    #[inline(always)]
    fn check_all(account_info: &AccountInfo, root_addr: &Pubkey, program_id: &Pubkey, id: u32) -> Result<(), u32> {
        Self::check_basic(account_info, root_addr, program_id)?;
        let tag = AccountTag::from_u8(Self::expected_tag()).unwrap();
        Self::check_id(tag, account_info, id)?;
        Ok(())
    }
}