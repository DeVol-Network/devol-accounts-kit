

pub const CLIENT_LP_WORKER_ID_OFFSET: usize = 0;
pub const CLIENT_LP_TASK_ID_OFFSET: usize = 4;
pub const CLIENT_LP_COUNTER_OFFSET: usize = 8;
pub const CLIENT_LP_ORIG_COUNTER_OFFSET: usize = 16;
pub const CLIENT_LP_TIME_OFFSET: usize = 24;
pub const CLIENT_LP_PS_OFFSET: usize = 32;
pub const CLIENT_LP_PS_COST_OFFSET: usize = 36;
pub const CLIENT_LP_PS_RESULT_OFFSET: usize = 44;
pub const CLIENT_LP_PS_TRADE_QTY_OFFSET: usize = 52;
pub const CLIENT_LP_PS_TRADE_COST_OFFSET: usize = 56;
pub const CLIENT_LP_SIZE: usize = 64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClientLp {
    pub worker_id: u32,
    pub task_id: u32,
    pub counter: i64,
    pub orig_counter: i64,
    pub time: i64,
    pub ps: u32,
    pub ps_cost: [u8; 8],
    pub ps_result: [u8; 8],
    pub ps_trade_qty: i32,
    pub ps_trade_cost: [u8; 8],
}

impl ClientLp {
    #[inline(always)]
    pub fn get_ps_cost(&self) -> i64 { i64::from_ne_bytes(self.ps_cost) }

    #[inline(always)]
    pub fn set_ps_cost(&mut self, value: i64) { self.ps_cost = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_ps_result(&self) -> i64 { i64::from_ne_bytes(self.ps_result) }

    #[inline(always)]
    pub fn set_ps_result(&mut self, value: i64) { self.ps_result = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_ps_trade_cost(&self) -> i64 { i64::from_ne_bytes(self.ps_trade_cost) }

    #[inline(always)]
    pub fn set_ps_trade_cost(&mut self, value: i64) { self.ps_trade_cost = value.to_ne_bytes() }
}

#[cfg(test)]
impl Default for ClientLp {
    fn default() -> Self {
        Self {
            worker_id: 0,
            task_id: 0,
            counter: 0,
            orig_counter: 0,
            time: 0,
            ps: 0,
            ps_cost: [0; 8],
            ps_result: [0; 8],
            ps_trade_qty: 0,
            ps_trade_cost: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::client::client_account::client_lp::*;

    #[test]
    fn test_client_lp_offsets() {
        let client_lp = ClientLp::default();

        let base_ptr = &client_lp as *const _ as usize;

        assert_eq!(&client_lp.worker_id as *const _ as usize - base_ptr, CLIENT_LP_WORKER_ID_OFFSET);
        assert_eq!(&client_lp.task_id as *const _ as usize - base_ptr, CLIENT_LP_TASK_ID_OFFSET);
        assert_eq!(&client_lp.counter as *const _ as usize - base_ptr, CLIENT_LP_COUNTER_OFFSET);
        assert_eq!(&client_lp.orig_counter as *const _ as usize - base_ptr, CLIENT_LP_ORIG_COUNTER_OFFSET);
        assert_eq!(&client_lp.time as *const _ as usize - base_ptr, CLIENT_LP_TIME_OFFSET);
        assert_eq!(&client_lp.ps as *const _ as usize - base_ptr, CLIENT_LP_PS_OFFSET);
        assert_eq!(&client_lp.ps_cost as *const _ as usize - base_ptr, CLIENT_LP_PS_COST_OFFSET);
        assert_eq!(&client_lp.ps_result as *const _ as usize - base_ptr, CLIENT_LP_PS_RESULT_OFFSET);
        assert_eq!(&client_lp.ps_trade_qty as *const _ as usize - base_ptr, CLIENT_LP_PS_TRADE_QTY_OFFSET);
        assert_eq!(&client_lp.ps_trade_cost as *const _ as usize - base_ptr, CLIENT_LP_PS_TRADE_COST_OFFSET);

        assert_eq!(std::mem::size_of::<ClientLp>(), CLIENT_LP_SIZE);
    }
}
