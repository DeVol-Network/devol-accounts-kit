use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::tasks_trace::tasks_trace::TasksTrace;

pub const TASKS_TRACE_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const TASKS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const TASKS_TRACE_ACCOUNT_WORKER_ID_OFFSET: usize = 40;
pub const TASKS_TRACE_ACCOUNT_DATA_OFFSET: usize = 44;
pub const TASKS_TRACE_ACCOUNT_SIZE: usize = 7724;
pub const TASKS_TRACE_ACCOUNT_TAG: u8 = 5;
pub const TASKS_TRACE_ACCOUNT_VERSION: u32 = 7;
pub const MAX_TASKS_TRACE_COUNT: usize = 128;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TasksTraceAccount {
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    // 4 bytes, TASKS_TRACE_ACCOUNT_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 7680 bytes, TASKS_TRACE_ACCOUNT_DATA_OFFSET
    pub data: [TasksTrace; MAX_TASKS_TRACE_COUNT],
}

impl DevolIndexedAccount for TasksTraceAccount{}

impl DevolAccount for TasksTraceAccount {
    #[inline(always)]
    fn expected_size() -> usize { TASKS_TRACE_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 {
        TASKS_TRACE_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 {
        TASKS_TRACE_ACCOUNT_VERSION
    }
}

#[cfg(test)]
impl Default for TasksTraceAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            worker_id: 0,
            data: [TasksTrace::default(); MAX_TASKS_TRACE_COUNT],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_trace_account_offsets() {
        let account = TasksTraceAccount::default();
        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &account.header.tag as *const _ as usize - base_ptr,
            TASKS_TRACE_ACCOUNT_VERSION_OFFSET
        );
        assert_eq!(
            &account.header.root as *const _ as usize - base_ptr,
            TASKS_TRACE_ACCOUNT_ROOT_ADDRESS_OFFSET
        );
        assert_eq!(
            &account.worker_id as *const _ as usize - base_ptr,
            TASKS_TRACE_ACCOUNT_WORKER_ID_OFFSET
        );
        assert_eq!(
            &account.data as *const _ as usize - base_ptr,
            TASKS_TRACE_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TasksTraceAccount>(), TASKS_TRACE_ACCOUNT_SIZE);
    }
}
