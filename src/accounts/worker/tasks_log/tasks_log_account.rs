use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::tasks_log::task_log::TasksLog;

pub const TASKS_LOG_BUFFER_CAPACITY: usize = 256;
pub const TASKS_LOG_ACCOUNT_VERSION_OFFSET: usize = 0;
pub const TASKS_LOG_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const TASKS_LOG_ACCOUNT_WORKER_ID_OFFSET: usize = 40;
pub const TASKS_LOG_ACCOUNT_LAST_OFFSET: usize = 44;
pub const TASKS_LOG_ACCOUNT_COUNT_OFFSET: usize = 48;
pub const TASKS_LOG_ACCOUNT_DATA_OFFSET: usize = 52;
pub const TASKS_LOG_ACCOUNT_SIZE: usize = 26676;
pub const TASKS_LOG_ACCOUNT_TAG: u8 = 11;
pub const TASKS_LOG_ACCOUNT_VERSION: u32 = 13;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TasksLogAccount {
    // 40 bytes, AccountHeader
    pub header: AccountHeader,
    // 4 bytes, TASKS_LOG_ACCOUNT_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 4 bytes, TASKS_LOG_ACCOUNT_LAST_OFFSET
    pub last: u32,
    // 4 bytes, TASKS_LOG_ACCOUNT_COUNT_OFFSET
    pub count: u32,
    // 26624 bytes, TASKS_LOG_ACCOUNT_DATA_OFFSET
    pub data: [TasksLog; TASKS_LOG_BUFFER_CAPACITY],
}
impl DevolIndexedAccount for TasksLogAccount{}

impl DevolAccount for TasksLogAccount {
    #[inline(always)]
    fn expected_size() -> usize { TASKS_LOG_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 {
        TASKS_LOG_ACCOUNT_TAG
    }

    #[inline(always)]
    fn expected_version() -> u32 {
        TASKS_LOG_ACCOUNT_VERSION
    }
}

#[cfg(test)]
impl Default for TasksLogAccount {
    fn default() -> Self {
        Self {
            header: AccountHeader::default(),
            worker_id: 0,
            last: 0,
            count: 0,
            data: [TasksLog::default(); TASKS_LOG_BUFFER_CAPACITY],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_log_account_offsets() {
        let account = TasksLogAccount::default();
        let base_ptr = &account as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &account.header as *const _ as usize - base_ptr,
            0
        );
        assert_eq!(
            &account.worker_id as *const _ as usize - base_ptr,
            TASKS_LOG_ACCOUNT_WORKER_ID_OFFSET
        );
        assert_eq!(
            &account.last as *const _ as usize - base_ptr,
            TASKS_LOG_ACCOUNT_LAST_OFFSET
        );
        assert_eq!(
            &account.count as *const _ as usize - base_ptr,
            TASKS_LOG_ACCOUNT_COUNT_OFFSET
        );
        assert_eq!(
            &account.data as *const _ as usize - base_ptr,
            TASKS_LOG_ACCOUNT_DATA_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<TasksLogAccount>(), TASKS_LOG_ACCOUNT_SIZE);
    }
}
