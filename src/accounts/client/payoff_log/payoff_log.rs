use std::error::Error;

pub const PAYOFF_LOG_ID_OFFSET: usize = 0;
pub const PAYOFF_LOG_WORKER_ID_OFFSET: usize = 8;
pub const PAYOFF_LOG_POOL_ID_OFFSET: usize = 12;
pub const PAYOFF_LOG_INSTR_ID_OFFSET: usize = 16;
pub const PAYOFF_LOG_TRADE_TIME_OFFSET: usize = 20;
pub const PAYOFF_LOG_UPDATE_TIME_OFFSET: usize = 28;
pub const PAYOFF_LOG_SETTLEMENT_PX_OFFSET: usize = 36;
pub const PAYOFF_LOG_PAYOFF_OFFSET: usize = 44;
pub const PAYOFF_LOG_UNBLOCKED_COLLATERAL_OFFSET: usize = 52;
pub const PAYOFF_LOG_RESULT_OFFSET: usize = 60;
pub const PAYOFF_LOG_SIZE: usize = 68;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PayoffLog {
    pub id: [u8; 8],    // 8 bytes, PAYOFF_LOG_ID_OFFSET
    // 4 bytes, PAYOFF_LOG_WORKER_ID_OFFSET
    pub worker_id: u32,
    // 4 bytes, PAYOFF_LOG_POOL_ID_OFFSET
    pub pool_id: u32,
    // 4 bytes, PAYOFF_LOG_INSTR_ID_OFFSET
    pub instr_id: u32,
    // 8 bytes, PAYOFF_LOG_TRADE_TIME_OFFSET
    pub trade_time: [u8; 8],
    // 8 bytes, PAYOFF_LOG_UPDATE_TIME_OFFSET
    pub update_time: [u8; 8],
    // 8 bytes, PAYOFF_LOG_SETTLEMENT_PX_OFFSET
    pub settlement_px: [u8; 8],
    // 8 bytes, PAYOFF_LOG_PAYOFF_OFFSET
    pub payoff: [u8; 8],
    // 8 bytes, PAYOFF_LOG_UNBLOCKED_COLLATERAL_OFFSET
    pub unblocked_collateral: [u8; 8],
    // 8 bytes, PAYOFF_LOG_RESULT_OFFSET
    pub result: [u8; 8],
}

impl PayoffLog {
    #[inline(always)]
    pub fn get_id(&self) -> i64 { i64::from_ne_bytes(self.id) }
    #[inline(always)]
    pub fn set_id(&mut self, value: i64) { self.id = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_trade_time(&self) -> i64 { i64::from_ne_bytes(self.trade_time) }
    #[inline(always)]
    pub fn set_trade_time(&mut self, value: i64) { self.trade_time = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_update_time(&self) -> i64 { i64::from_ne_bytes(self.update_time) }
    #[inline(always)]
    pub fn set_update_time(&mut self, value: i64) { self.update_time = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_settlement_px(&self) -> i64 { i64::from_ne_bytes(self.settlement_px) }
    #[inline(always)]
    pub fn set_settlement_px(&mut self, value: i64) { self.settlement_px = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_payoff(&self) -> i64 { i64::from_ne_bytes(self.payoff) }
    #[inline(always)]
    pub fn set_payoff(&mut self, value: i64) { self.payoff = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_unblocked_collateral(&self) -> i64 { i64::from_ne_bytes(self.unblocked_collateral) }
    #[inline(always)]
    pub fn set_unblocked_collateral(&mut self, value: i64) { self.unblocked_collateral = value.to_ne_bytes(); }

    #[inline(always)]
    pub fn get_result(&self) -> i64 { i64::from_ne_bytes(self.result) }
    #[inline(always)]
    pub fn set_result(&mut self, value: i64) { self.result = value.to_ne_bytes(); }
}

impl Default for PayoffLog {
    fn default() -> Self {
        Self {
            id: [0; 8],
            worker_id: 0,
            pool_id: 0,
            instr_id: 0,
            trade_time: [0; 8],
            update_time: [0; 8],
            settlement_px: [0; 8],
            payoff: [0; 8],
            unblocked_collateral: [0; 8],
            result: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use crate::utils::type_size_helper::align_size;

    #[test]
    fn test_payoff_log_offsets_and_sizes() {
        let log = PayoffLog::default();

        let base_ptr = &log as *const _ as usize;

        // checking fields size and offset
        assert_eq!(unsafe { &log.id as *const _ as usize } - base_ptr, PAYOFF_LOG_ID_OFFSET);
        assert_eq!(unsafe { &log.worker_id as *const _ as usize } - base_ptr, PAYOFF_LOG_WORKER_ID_OFFSET);
        assert_eq!(unsafe { &log.pool_id as *const _ as usize } - base_ptr, PAYOFF_LOG_POOL_ID_OFFSET);
        assert_eq!(unsafe { &log.instr_id as *const _ as usize } - base_ptr, PAYOFF_LOG_INSTR_ID_OFFSET);
        assert_eq!(unsafe { &log.trade_time as *const _ as usize } - base_ptr, PAYOFF_LOG_TRADE_TIME_OFFSET);
        assert_eq!(unsafe { &log.update_time as *const _ as usize } - base_ptr, PAYOFF_LOG_UPDATE_TIME_OFFSET);
        assert_eq!(unsafe { &log.settlement_px as *const _ as usize } - base_ptr, PAYOFF_LOG_SETTLEMENT_PX_OFFSET);
        assert_eq!(unsafe { &log.payoff as *const _ as usize } - base_ptr, PAYOFF_LOG_PAYOFF_OFFSET);
        assert_eq!(unsafe { &log.unblocked_collateral as *const _ as usize } - base_ptr, PAYOFF_LOG_UNBLOCKED_COLLATERAL_OFFSET);
        assert_eq!(unsafe { &log.result as *const _ as usize } - base_ptr, PAYOFF_LOG_RESULT_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<PayoffLog>(), align_size(PAYOFF_LOG_SIZE, 4));
    }
}