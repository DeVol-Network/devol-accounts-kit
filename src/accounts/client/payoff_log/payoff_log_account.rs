use std::error::Error;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::accounts::account_header::AccountHeader;
use crate::accounts::client::payoff_log::payoff_log::PayoffLog;
use crate::accounts::devol_account::DevolAccount;
use crate::errors::AccountTag;

pub const PAYOFF_LOG_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const PAYOFF_LOG_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const PAYOFF_LOG_ACCOUNT_ID_OFFSET: usize = 40;
pub const PAYOFF_LOG_ACCOUNT_LAST_OFFSET: usize = 44;
pub const PAYOFF_LOG_ACCOUNT_COUNT_OFFSET: usize = 48;
pub const PAYOFF_LOG_ACCOUNT_DATA_OFFSET: usize = 52;
pub const PAYOFF_LOG_ACCOUNT_SIZE: usize = 8756;
pub const PAYOFF_LOG_ACCOUNT_TAG: u8 = 9;
pub const PAYOFF_LOG_ACCOUNT_VERSION: u32 = 11;
pub const PAYOFF_LOG_BUFFER_CAPACITY: usize = 128;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PayoffLogAccount {
    pub header: AccountHeader, // 40 bytes
    pub id: u32, // 4 bytes
    pub last: u32, // 4 bytes
    pub count: u32, // 4 bytes
    pub data: [PayoffLog; PAYOFF_LOG_BUFFER_CAPACITY],
}

impl DevolAccount for PayoffLogAccount {
    #[inline(always)]
    fn expected_size() -> usize { PAYOFF_LOG_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 { PAYOFF_LOG_ACCOUNT_TAG }

    #[inline(always)]
    fn expected_version() -> u32 { PAYOFF_LOG_ACCOUNT_VERSION }
}

impl Default for PayoffLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            id: 0,
            last: 0,
            count: 0,
            data: [PayoffLog::default(); PAYOFF_LOG_BUFFER_CAPACITY],
        }
    }
}

// pub fn check_client_results_log_account(
//     account: &AccountInfo,
//     root_key: Pubkey,
//     program_id: &Pubkey,
//     id: u32,
// ) -> bool {
//     if account.owner != program_id
//         || !account.is_writable
//         || payoff_log_account_id!(account) != id
//         || unsafe { *(account.data.borrow().as_ptr() as *const i64) }
//         != version!(PAYOFF_LOG_ACCOUNT_VERSION, PAYOFF_LOG_ACCOUNT_TAG)
//     {
//         return false;
//     }
//     return unsafe { *(account.data.borrow()[8..40].as_ptr() as *const Pubkey) } == root_key;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_root_account_offsets() {
        let account = PayoffLogAccount::default();

        for i in 0..PAYOFF_LOG_BUFFER_CAPACITY {
            let log = &account.data[i];
            let log_ptr = log as *const _;
            let expected_ptr = account.data.as_ptr() as usize + i * mem::size_of::<PayoffLog>();
            assert_eq!(log_ptr as usize, expected_ptr, "PayoffLog offset is incorrect for index {}", i);
        }

        let base_ptr = &account as *const _ as usize;
        // checking fields size and offset
        assert_eq!(unsafe { &account.header as *const _ as usize } - base_ptr, PAYOFF_LOG_ACCOUNT_VERSION_OFFSET);
        assert_eq!(unsafe { &account.id as *const _ as usize } - base_ptr, PAYOFF_LOG_ACCOUNT_ID_OFFSET);
        assert_eq!(unsafe { &account.last as *const _ as usize } - base_ptr, PAYOFF_LOG_ACCOUNT_LAST_OFFSET);
        assert_eq!(unsafe { &account.count as *const _ as usize } - base_ptr, PAYOFF_LOG_ACCOUNT_COUNT_OFFSET);
        assert_eq!(unsafe { &account.data as *const _ as usize } - base_ptr, PAYOFF_LOG_ACCOUNT_DATA_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<PayoffLogAccount>(), PAYOFF_LOG_ACCOUNT_SIZE);
    }
}
