use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_log::pool_log::PoolsLog;

pub const POOLS_LOG_BUFFER_CAPACITY: usize = 256;
pub const POOLS_LOG_ACCOUNT_SIZE: usize = 0;
pub const POOLS_LOG_ACCOUNT_TAG: u8 = 6;
pub const POOLS_LOG_ACCOUNT_VERSION: u32 = 9;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PoolsLogAccount {
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    pub worker_id: u32,
    pub last_pool_id: u32,
    pub pools_count: u32,
    reserved: u32, // Reserved to fit the alignment 64 bit
    pub data: [PoolsLog; POOLS_LOG_BUFFER_CAPACITY],
}

impl DevolIndexedAccount for PoolsLogAccount {}

impl DevolAccount for PoolsLogAccount {
    fn expected_size() -> usize { POOLS_LOG_ACCOUNT_SIZE }

    fn expected_tag() -> u8 {
        POOLS_LOG_ACCOUNT_TAG
    }

    fn expected_version() -> u32 {
        POOLS_LOG_ACCOUNT_VERSION
    }
}

#[cfg(test)]
impl Default for PoolsLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            worker_id: 0,
            last_pool_id: 0,
            pools_count: 0,
            reserved: 0,
            data: [PoolsLog::default(); POOLS_LOG_BUFFER_CAPACITY],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_log_account_offsets() {
        let account = PoolsLogAccount::default();
        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &account.header as *const _ as usize - base_ptr,
            0
        );

        // checking total size
        assert_eq!(std::mem::size_of::<PoolsLogAccount>(), POOLS_LOG_ACCOUNT_SIZE);
    }
}