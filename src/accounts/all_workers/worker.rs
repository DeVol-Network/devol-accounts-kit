use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

pub const WORKER_ADDRESS_OFFSET: usize = 0;
pub const WORKER_POOLS_TRACE_ADDRESS_OFFSET: usize = 32;
pub const WORKER_TASKS_TRACE_ADDRESS_OFFSET: usize = 64;
pub const WORKER_POOLS_LOG_ADDRESS_OFFSET: usize = 96;
pub const WORKER_TASKS_LOG_ADDRESS_OFFSET: usize = 128;
pub const WORKER_SIZE: usize = 160;

#[derive(Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Worker {
    pub address: Pubkey,
    pub pools_trace_address: Pubkey,
    pub tasks_trace_address: Pubkey,
    pub pools_log_address: Pubkey,
    pub tasks_log_address: Pubkey,
}

#[cfg(test)]
impl Default for Worker {
    fn default() -> Self {
        Self {
            address: Pubkey::default(),
            pools_trace_address: Pubkey::default(),
            tasks_trace_address: Pubkey::default(),
            pools_log_address: Pubkey::default(),
            tasks_log_address: Pubkey::default(),
        }
    }
}
