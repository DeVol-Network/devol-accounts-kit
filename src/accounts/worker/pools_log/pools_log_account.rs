use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::pools_log::pool_log::PoolsLog;

pub const POOLS_LOG_BUFFER_CAPACITY: usize = 256;
pub const POOLS_LOG_ACCOUNT_SIZE: usize = 335928;
pub const POOLS_LOG_ACCOUNT_TAG: u8 = 6;
pub const POOLS_LOG_ACCOUNT_VERSION: u32 = 9;

#[repr(C)]
#[derive(Clone, Copy)]
// Pools log account. Alignment - 64 bit.
pub struct PoolsLogAccount {
    pub header: AccountHeader,  // 40 bytes
    pub worker_id: u32,         // 4 bytes (1/2 align)
    pub last_pool_id: u32,      // 4 bytes (2/2 align)
    pub pools_count: u32,       // 4 bytes (1/2 align)
    reserved: u32,              // 4 bytes (2/2 align)
    pub data: [PoolsLog; POOLS_LOG_BUFFER_CAPACITY], // 1312x256=335872 bytes
}

impl DevolIndexedAccount for PoolsLogAccount {}

impl DevolAccount for PoolsLogAccount {
    #[inline(always)]
    fn expected_size() -> usize { POOLS_LOG_ACCOUNT_SIZE }

    #[inline(always)]
    fn expected_tag() -> u8 {
        POOLS_LOG_ACCOUNT_TAG
    }

    #[inline(always)]
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
    use crate::utils::type_size_helper::align_size;
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
        let real_size = std::mem::size_of::<PoolsLogAccount>();
        assert_eq!(real_size, POOLS_LOG_ACCOUNT_SIZE);
        assert_eq!(real_size, align_size(real_size, 8));
    }
}