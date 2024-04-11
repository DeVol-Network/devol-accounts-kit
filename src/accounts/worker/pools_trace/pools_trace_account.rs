use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_trace::pools_trace::PoolsTrace;
use crate::accounts::worker::tasks_trace::tasks_trace_account::TasksTraceAccount;

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
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    // 4 bytes, POOLS_TRACE_ACCOUNT_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 24576 bytes, POOLS_TRACE_ACCOUNT_DATA_OFFSET
    pub data: [PoolsTrace; MAX_POOLS_TRACE_COUNT],
}
impl DevolIndexedAccount for PoolsTraceAccount{}

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
            header: AccountHeader::default(),
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
            &account.header.tag as *const _ as usize - base_ptr,
            POOLS_TRACE_ACCOUNT_VERSION_OFFSET
        );
        assert_eq!(
            &account.header.root as *const _ as usize - base_ptr,
            POOLS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET
        );
        assert_eq!(
            &account.worker_id as *const _ as usize - base_ptr,
            POOLS_TRACE_ACCOUNT_WORKER_ID_OFFSET
        );
        assert_eq!(
            &account.data as *const _ as usize - base_ptr,
            POOLS_TRACE_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<PoolsTraceAccount>(), POOLS_TRACE_ACCOUNT_SIZE);
    }
}
