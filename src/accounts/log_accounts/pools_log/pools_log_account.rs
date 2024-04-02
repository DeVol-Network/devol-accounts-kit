use crate::accounts::account_header::AccountHeader;
use crate::accounts::log_accounts::pools_log::pool_log::PoolsLog;

pub const POOLS_LOG_BUFFER_CAPACITY: usize = 256;
pub const POOLS_LOG_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const POOLS_LOG_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const POOLS_LOG_ACCOUNT_WORKER_ID_OFFSET: usize = 40;
pub const POOLS_LOG_ACCOUNT_LAST_OFFSET: usize = 44;
pub const POOLS_LOG_ACCOUNT_COUNT_OFFSET: usize = 48;
pub const POOLS_LOG_ACCOUNT_DATA_OFFSET: usize = 52;
pub const POOLS_LOG_ACCOUNT_SIZE: usize = 334132;
pub const POOLS_LOG_ACCOUNT_TAG: u8 = 6;
pub const POOLS_LOG_ACCOUNT_VERSION: usize = 8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PoolsLogAccount {
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    // 4 bytes, POOLS_LOG_ACCOUNT_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 4 bytes, POOLS_LOG_ACCOUNT_LAST_OFFSET
    pub last: u32,
    // 4 bytes, POOLS_LOG_ACCOUNT_COUNT_OFFSET
    pub count: u32,
    // 334080 bytes, POOLS_LOG_ACCOUNT_DATA_OFFSET
    pub data: [PoolsLog; POOLS_LOG_BUFFER_CAPACITY],
}

impl Default for PoolsLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            worker_id: 0,
            last: 0,
            count: 0,
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
            unsafe { &account.header as *const _ as usize } - base_ptr,
            0
        );
        assert_eq!(
            unsafe { &account.worker_id as *const _ as usize } - base_ptr,
            POOLS_LOG_ACCOUNT_WORKER_ID_OFFSET
        );
        assert_eq!(
            unsafe { &account.last as *const _ as usize } - base_ptr,
            POOLS_LOG_ACCOUNT_LAST_OFFSET
        );
        assert_eq!(
            unsafe { &account.count as *const _ as usize } - base_ptr,
            POOLS_LOG_ACCOUNT_COUNT_OFFSET
        );
        assert_eq!(
            unsafe { &account.data as *const _ as usize } - base_ptr,
            POOLS_LOG_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<PoolsLogAccount>(), POOLS_LOG_ACCOUNT_SIZE);
    }
}