use crate::constants::FD;

pub const LP_PORTFOLIO_RECORD_SIZE: usize = 64;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
/// 8 bytes alignment
pub struct LpPortfolioPoolRecord {
    pub worker_id: u32,     // 4 bytes (4/8 bytes align)
    pub task_id: u32,       // 4 bytes (8/8 bytes align)
    pub client_lp_trades_counter: i64,        // 8 bytes
    pub pool_lp_trades_counter: i64, // 8 bytes
    pub last_trade_time: i64,          // 8 bytes
    pub ps: u32,            // 4 bytes (4/8 bytes align)
    pub ps_trade_qty: i32,  // 4 bytes (8/8 bytes align)
    pub ps_trade_cost: i64, // 8 bytes
    pub ps_cost: i64,       // 8 bytes
    pub ps_result: i64,     // 8 bytes
}

impl LpPortfolioPoolRecord {
    #[inline(always)]
    pub fn get_ps_cost_f64(&self) -> f64 { self.ps_cost as f64 / FD }
    #[inline(always)]
    pub fn get_ps_result_f64(&self) -> f64 { self.ps_result as f64 / FD }
    #[inline(always)]
    pub fn get_ps_trade_cost_f64(&self) -> f64 { self.ps_trade_cost as f64 / FD }
}

impl Default for LpPortfolioPoolRecord {
    fn default() -> Self {
        Self {
            worker_id: 0,
            task_id: 0,
            client_lp_trades_counter: 0,
            pool_lp_trades_counter: 0,
            last_trade_time: 0,
            ps: 0,
            ps_trade_qty: 0,
            ps_trade_cost: 0,
            ps_cost: 0,
            ps_result: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts::client::lp_portfolio_account::lp_portfolio_pool_record::{LP_PORTFOLIO_RECORD_SIZE, LpPortfolioPoolRecord};

    #[test]
    fn test_client_lp_offsets() {
        assert_eq!(std::mem::size_of::<LpPortfolioPoolRecord>(), LP_PORTFOLIO_RECORD_SIZE);
    }
}
