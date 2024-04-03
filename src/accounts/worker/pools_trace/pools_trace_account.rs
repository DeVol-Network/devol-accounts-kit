use solana_program::pubkey::Pubkey;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::worker::pools_trace::pools_trace::PoolsTrace;

pub const POOLS_TRACE_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const POOLS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const POOLS_TRACE_ACCOUNT_WORKER_ID_OFFSET: usize = 40;
pub const POOLS_TRACE_ACCOUNT_DATA_OFFSET: usize = 44;
pub const POOLS_TRACE_ACCOUNT_SIZE: usize = 24620;
pub const POOLS_TRACE_ACCOUNT_TAG: u8 = 4;
pub const POOLS_TRACE_ACCOUNT_VERSION: u32 = 6;
pub const MAX_POOLS_TRACE_COUNT: usize = 256;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PoolsTraceAccount {
    // 8 bytes, POOLS_TRACE_ACCOUNT_VERSION_OFFSET
    pub version: [u8; 8],
    // 32 bytes, POOLS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET
    pub root_address: Pubkey,
    // 4 bytes, POOLS_TRACE_ACCOUNT_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 24576 bytes, POOLS_TRACE_ACCOUNT_DATA_OFFSET
    pub data: [PoolsTrace; MAX_POOLS_TRACE_COUNT],
}

impl PoolsTraceAccount {
    #[inline(always)]
    pub fn get_version(&self) -> i64 {
        i64::from_ne_bytes(self.version)
    }

    #[inline(always)]
    pub fn set_version(&mut self, value: i64) {
        self.version = value.to_ne_bytes();
    }
}

impl DevolAccount for PoolsTraceAccount {
    fn expected_size() -> usize { POOLS_TRACE_ACCOUNT_SIZE }

    fn expected_tag() -> u8 {
        POOLS_TRACE_ACCOUNT_TAG
    }

    fn expected_version() -> u32 {
        POOLS_TRACE_ACCOUNT_VERSION
    }
}

impl Default for PoolsTraceAccount {
    fn default() -> Self {
        Self {
            version: [0; 8],
            root_address: Pubkey::default(),
            worker_id: 0,
            data: [PoolsTrace::default(); MAX_POOLS_TRACE_COUNT],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_trace_account_offsets() {
        let account = PoolsTraceAccount::default();
        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            unsafe { &account.version as *const _ as usize } - base_ptr,
            POOLS_TRACE_ACCOUNT_VERSION_OFFSET
        );
        assert_eq!(
            unsafe { &account.root_address as *const _ as usize } - base_ptr,
            POOLS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET
        );
        assert_eq!(
            unsafe { &account.worker_id as *const _ as usize } - base_ptr,
            POOLS_TRACE_ACCOUNT_WORKER_ID_OFFSET
        );
        assert_eq!(
            unsafe { &account.data as *const _ as usize } - base_ptr,
            POOLS_TRACE_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<PoolsTraceAccount>(), POOLS_TRACE_ACCOUNT_SIZE);
    }
}
