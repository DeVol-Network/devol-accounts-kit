pub const POOLS_TRACE_TASK_ID_OFFSET: usize = 0;
pub const POOLS_TRACE_EXPIRATION_OFFSET: usize = 4;
pub const POOLS_TRACE_UPDATE_TIME_OFFSET: usize = 12;
pub const POOLS_TRACE_SETTLEMENT_PX_OFFSET: usize = 20;
pub const POOLS_TRACE_CPS_PX_OFFSET: usize = 28;
pub const POOLS_TRACE_PS_OFFSET: usize = 36;
pub const POOLS_TRACE_PS_PX_OFFSET: usize = 40;
pub const POOLS_TRACE_TASK_FEES_OFFSET: usize = 48;
pub const POOLS_TRACE_PAYOFF_LONG_OFFSET: usize = 56;
pub const POOLS_TRACE_PAYOFF_SHORT_OFFSET: usize = 64;
pub const POOLS_TRACE_REST_OF_PAYOFF_LONG_OFFSET: usize = 72;
pub const POOLS_TRACE_REST_OF_PAYOFF_SHORT_OFFSET: usize = 80;
pub const POOLS_TRACE_SETTLEMENT_STRIKE_OFFSET: usize = 88;
pub const POOLS_TRACE_SIZE: usize = 96;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PoolsTrace {
    // 4 bytes, POOLS_TRACE_TASK_ID_OFFSET
    pub task_id: u32,
    // 8 bytes, POOLS_TRACE_EXPIRATION_OFFSET
    pub expiration: [u8; 8],
    // 8 bytes, POOLS_TRACE_UPDATE_TIME_OFFSET
    pub update_time: [u8; 8],
    // 8 bytes, POOLS_TRACE_SETTLEMENT_PX_OFFSET
    pub settlement_px: [u8; 8],
    // 8 bytes, POOLS_TRACE_CPS_PX_OFFSET
    pub cps_px: [u8; 8],
    // 4 bytes, POOLS_TRACE_PS_OFFSET
    pub ps: u32,
    // 8 bytes, POOLS_TRACE_PS_PX_OFFSET
    pub ps_px: [u8; 8],
    // 8 bytes, POOLS_TRACE_TASK_FEES_OFFSET
    pub task_fees: [u8; 8],
    // 8 bytes, POOLS_TRACE_PAYOFF_LONG_OFFSET
    pub payoff_long: [u8; 8],
    // 8 bytes, POOLS_TRACE_PAYOFF_SHORT_OFFSET
    pub payoff_short: [u8; 8],
    // 8 bytes, POOLS_TRACE_REST_OF_PAYOFF_LONG_OFFSET
    pub rest_of_payoff_long: [u8; 8],
    // 8 bytes, POOLS_TRACE_REST_OF_PAYOFF_SHORT_OFFSET
    pub rest_of_payoff_short: [u8; 8],
    // 8 bytes, POOLS_TRACE_SETTLEMENT_STRIKE_OFFSET
    pub payoff_time: [u8; 8],
}

impl PoolsTrace {
    #[inline(always)]
    pub fn get_expiration(&self) -> i64 {
        i64::from_ne_bytes(self.expiration)
    }

    #[inline(always)]
    pub fn set_expiration(&mut self, value: i64) {
        self.expiration = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_update_time(&self) -> i64 {
        i64::from_ne_bytes(self.update_time)
    }

    #[inline(always)]
    pub fn set_update_time(&mut self, value: i64) {
        self.update_time = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_settlement_px(&self) -> i64 {
        i64::from_ne_bytes(self.settlement_px)
    }

    #[inline(always)]
    pub fn set_settlement_px(&mut self, value: i64) {
        self.settlement_px = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_cps_px(&self) -> i64 {
        i64::from_ne_bytes(self.cps_px)
    }

    #[inline(always)]
    pub fn set_cps_px(&mut self, value: i64) {
        self.cps_px = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_ps_px(&self) -> i64 {
        i64::from_ne_bytes(self.ps_px)
    }

    #[inline(always)]
    pub fn set_ps_px(&mut self, value: i64) {
        self.ps_px = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_task_fees(&self) -> i64 {
        i64::from_ne_bytes(self.task_fees)
    }

    #[inline(always)]
    pub fn set_task_fees(&mut self, value: i64) {
        self.task_fees = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_payoff_long(&self) -> i64 {
        i64::from_ne_bytes(self.payoff_long)
    }

    #[inline(always)]
    pub fn set_payoff_long(&mut self, value: i64) {
        self.payoff_long = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_payoff_short(&self) -> i64 {
        i64::from_ne_bytes(self.payoff_short)
    }

    #[inline(always)]
    pub fn set_payoff_short(&mut self, value: i64) {
        self.payoff_short = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_rest_of_payoff_long(&self) -> i64 {
        i64::from_ne_bytes(self.rest_of_payoff_long)
    }

    #[inline(always)]
    pub fn set_rest_of_payoff_long(&mut self, value: i64) {
        self.rest_of_payoff_long = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_rest_of_payoff_short(&self) -> i64 {
        i64::from_ne_bytes(self.rest_of_payoff_short)
    }

    #[inline(always)]
    pub fn set_rest_of_payoff_short(&mut self, value: i64) {
        self.rest_of_payoff_short = value.to_ne_bytes();
    }

    #[inline(always)]
    pub fn get_payoff_time(&self) -> i64 {
        i64::from_ne_bytes(self.payoff_time)
    }

    #[inline(always)]
    pub fn set_payoff_time(&mut self, value: i64) {
        self.payoff_time = value.to_ne_bytes();
    }
}

impl Default for PoolsTrace {
    fn default() -> Self {
        Self {
            task_id: 0,
            expiration: [0; 8],
            update_time: [0; 8],
            settlement_px: [0; 8],
            cps_px: [0; 8],
            ps: 0,
            ps_px: [0; 8],
            task_fees: [0; 8],
            payoff_long: [0; 8],
            payoff_short: [0; 8],
            rest_of_payoff_long: [0; 8],
            rest_of_payoff_short: [0; 8],
            payoff_time: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_trace_offsets() {
        let trace = PoolsTrace::default();
        let base_ptr = &trace as *const _ as usize;

        // checking fields size and offset
        assert_eq!(
            &trace.task_id as *const _ as usize - base_ptr,
            POOLS_TRACE_TASK_ID_OFFSET
        );
        assert_eq!(
            &trace.expiration as *const _ as usize - base_ptr,
            POOLS_TRACE_EXPIRATION_OFFSET
        );
        assert_eq!(
            &trace.update_time as *const _ as usize - base_ptr,
            POOLS_TRACE_UPDATE_TIME_OFFSET
        );
        assert_eq!(
            &trace.settlement_px as *const _ as usize - base_ptr,
            POOLS_TRACE_SETTLEMENT_PX_OFFSET
        );
        assert_eq!(
            &trace.cps_px as *const _ as usize - base_ptr,
            POOLS_TRACE_CPS_PX_OFFSET
        );
        assert_eq!(
            &trace.ps as *const _ as usize - base_ptr,
            POOLS_TRACE_PS_OFFSET
        );
        assert_eq!(
            &trace.ps_px as *const _ as usize - base_ptr,
            POOLS_TRACE_PS_PX_OFFSET
        );
        assert_eq!(
            &trace.task_fees as *const _ as usize - base_ptr,
            POOLS_TRACE_TASK_FEES_OFFSET
        );
        assert_eq!(
            &trace.payoff_long as *const _ as usize - base_ptr,
            POOLS_TRACE_PAYOFF_LONG_OFFSET
        );
        assert_eq!(
            &trace.payoff_short as *const _ as usize - base_ptr,
            POOLS_TRACE_PAYOFF_SHORT_OFFSET
        );
        assert_eq!(
            &trace.rest_of_payoff_long as *const _ as usize - base_ptr,
            POOLS_TRACE_REST_OF_PAYOFF_LONG_OFFSET
        );
        assert_eq!(
            &trace.rest_of_payoff_short as *const _ as usize - base_ptr,
            POOLS_TRACE_REST_OF_PAYOFF_SHORT_OFFSET
        );
        assert_eq!(
            &trace.payoff_time as *const _ as usize - base_ptr,
            POOLS_TRACE_SETTLEMENT_STRIKE_OFFSET
        );

        // checking total size
        assert_eq!(std::mem::size_of::<PoolsTrace>(), POOLS_TRACE_SIZE);
    }
}
