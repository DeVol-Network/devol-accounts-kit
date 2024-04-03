
pub const CLIENT_POOL_ID_OFFSET: usize = 0;
pub const CLIENT_POOL_WORKER_ID_OFFSET: usize = 4;
pub const CLIENT_POOL_INSTR_ID_OFFSET: usize = 8;
pub const CLIENT_POOL_START_OFFSET: usize = 12;
pub const CLIENT_POOL_EXPIRATION_OFFSET: usize = 20;
pub const CLIENT_POOL_FRACTIONS_OFFSET: usize = 28;
pub const CLIENT_POOL_COUNTER_OFFSET: usize = 32;
pub const CLIENT_POOL_ORIG_COUNTER_OFFSET: usize = 40;
pub const CLIENT_POOL_TIME_OFFSET: usize = 48;
pub const CLIENT_POOL_DEPO_OFFSET: usize = 56;
pub const CLIENT_POOL_COST_OFFSET: usize = 816;
pub const CLIENT_POOL_RESULT_OFFSET: usize = 1576;
pub const CLIENT_POOL_CALLS_OFFSET: usize = 2336;
pub const CLIENT_POOL_CALLS_COST_OFFSET: usize = 2716;
pub const CLIENT_POOL_CALLS_RESULT_OFFSET: usize = 3476;
pub const CLIENT_POOL_PUTS_OFFSET: usize = 4236;
pub const CLIENT_POOL_PUTS_COST_OFFSET: usize = 4616;
pub const CLIENT_POOL_PUTS_RESULT_OFFSET: usize = 5376;
pub const CLIENT_POOL_LAST_COST_OFFSET: usize = 6136;
pub const CLIENT_POOL_LAST_FEES_OFFSET: usize = 6144;
pub const CLIENT_POOL_LAST_TRADE_OFFSET: usize = 6152;
pub const CLIENT_POOL_VANILLA_MEMO_OFFSET: usize = 6532;
pub const CLIENT_POOL_VANILLA_COST_OFFSET: usize = 6581;
pub const CLIENT_POOL_LAST_PX_OFFSET: usize = 6613;
pub const CLIENT_POOL_STRIKES_OFFSET: usize = 7373;
pub const CLIENT_POOL_BOUNDS_OFFSET: usize = 8133;
pub const CLIENT_POOL_SIZE: usize = 8885;

struct ClientPool{

}