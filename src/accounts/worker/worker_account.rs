use solana_program::account_info::AccountInfo;
use crate::account_readers::dvl_readable::{DvlIndexParam, DvlParametrable};
use crate::accounts::account_header::AccountHeader;
use crate::accounts::devol_account::DevolAccount;
use crate::accounts::devol_indexed_account::DevolIndexedAccount;
use crate::accounts::worker::band::Band;
use crate::accounts::worker::svm_params::SvmParams;
use crate::accounts::worker::worker_state::WorkerState;
use crate::constants::{BOUNDS_COUNT, BUCKETS_COUNT};
use crate::dvl_error::DvlError;

pub const WORKER_ACCOUNT_TAG_OFFSET: usize = 0;
pub const WORKER_ACCOUNT_VERSION_OFFSET: usize = 4;
pub const WORKER_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const WORKER_ACCOUNT_ID_OFFSET: usize = 40;
pub const WORKER_ACCOUNT_STATE_OFFSET: usize = 44;
pub const WORKER_ACCOUNT_TASK_ID_OFFSET: usize = 48;
pub const WORKER_ACCOUNT_INSTR_ID_OFFSET: usize = 52;
pub const WORKER_ACCOUNT_OPS_COUNTER_OFFSET: usize = 56;
pub const WORKER_ACCOUNT_FIRST_TIME_OFFSET: usize = 64;
pub const WORKER_ACCOUNT_DURATION_OFFSET: usize = 72;
pub const WORKER_ACCOUNT_INIT_PX_OFFSET: usize = 76;
pub const WORKER_ACCOUNT_OWN_PS_OFFSET: usize = 84;
pub const WORKER_ACCOUNT_WIDTH_FACTOR_OFFSET: usize = 88;
pub const WORKER_ACCOUNT_FEE_RATE_OFFSET: usize = 96;
pub const WORKER_ACCOUNT_FEE_RATIO_OFFSET: usize = 104;
pub const WORKER_ACCOUNT_INVENTORIES_RATIO_OFFSET: usize = 112;
pub const WORKER_ACCOUNT_FRACTIONS_OFFSET: usize = 120;
pub const WORKER_ACCOUNT_MAX_ORDER_OFFSET: usize = 124;
pub const WORKER_ACCOUNT_NEW_WIDTH_FACTOR_OFFSET: usize = 132;
pub const WORKER_ACCOUNT_NEW_FEE_RATE_OFFSET: usize = 140;
pub const WORKER_ACCOUNT_NEW_FEE_RATIO_OFFSET: usize = 148;
pub const WORKER_ACCOUNT_NEW_INVENTORIES_RATIO_OFFSET: usize = 156;
pub const WORKER_ACCOUNT_NEW_FRACTIONS_OFFSET: usize = 164;
pub const WORKER_ACCOUNT_NEW_MAX_ORDER_OFFSET: usize = 168;
pub const WORKER_ACCOUNT_NEW_MAX_INVENTORIES_OFFSET: usize = 176;
pub const WORKER_ACCOUNT_POOL_TIME_OFFSET: usize = 184;
pub const WORKER_ACCOUNT_LP_COUNTER_OFFSET: usize = 192;
pub const WORKER_ACCOUNT_LAST_LP_TIME_OFFSET: usize = 200;
pub const WORKER_ACCOUNT_POOL_SIZE_OFFSET: usize = 208;
pub const WORKER_ACCOUNT_CPS_OFFSET: usize = 216;
pub const WORKER_ACCOUNT_CPS_PX_OFFSET: usize = 224;
pub const WORKER_ACCOUNT_CPS_IN_PS_OFFSET: usize = 232;
pub const WORKER_ACCOUNT_PS_OFFSET: usize = 240;
pub const WORKER_ACCOUNT_PS_PX_OFFSET: usize = 244;
pub const WORKER_ACCOUNT_TASK_FEES_OFFSET: usize = 252;
pub const WORKER_ACCOUNT_REST_OF_FEES_OFFSET: usize = 260;
pub const WORKER_ACCOUNT_POOL_ID_OFFSET: usize = 268;
pub const WORKER_ACCOUNT_START_OFFSET: usize = 272;
pub const WORKER_ACCOUNT_EXPIRATION_OFFSET: usize = 280;
pub const WORKER_ACCOUNT_POOL_CASHFLOW_OFFSET: usize = 288;
pub const WORKER_ACCOUNT_SETTLEMENT_PX_OFFSET: usize = 296;
pub const WORKER_ACCOUNT_POOL_DEPO_OFFSET: usize = 304;
pub const WORKER_ACCOUNT_POOL_FEES_OFFSET: usize = 312;
pub const WORKER_ACCOUNT_POOL_COUNTER_OFFSET: usize = 320;
pub const WORKER_ACCOUNT_POOL_STRIKES_OFFSET: usize = 328;
pub const WORKER_ACCOUNT_POOL_BOUNDS_OFFSET: usize = 1088;
pub const WORKER_ACCOUNT_POOL_DISTRIB_OFFSET: usize = 1840;
pub const WORKER_ACCOUNT_SVM_PARAMS_OFFSET: usize = 4880;
pub const WORKER_ACCOUNT_MARGIN_VEGA_OFFSET: usize = 4920;
pub const WORKER_ACCOUNT_MARGIN_VANNA_OFFSET: usize = 4928;
pub const WORKER_ACCOUNT_MARGIN_VOLGA_OFFSET: usize = 4936;
pub const WORKER_ACCOUNT_RANGE_LR_OFFSET: usize = 4944;
pub const WORKER_ACCOUNT_W_LR_OFFSET: usize = 4952;
pub const WORKER_ACCOUNT_MAX_LR_OFFSET: usize = 4960;
pub const WORKER_ACCOUNT_MAX_PCT_POOL_OFFSET: usize = 4968;
pub const WORKER_PERM_IMPACT_OFFSET: usize = 4976;
pub const WORKER_ACCOUNT_SIZE: usize = 4984;
pub const WORKER_ACCOUNT_TAG: u8 = 7;
pub const WORKER_ACCOUNT_VERSION: u32 = 14;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WorkerAccount {
    pub header: AccountHeader,              //  40 bytes
    pub id: u32,                            //  4 bytes, WORKER_ACCOUNT_ID_OFFSET
    pub state: WorkerState,                 //  4 bytes, WORKER_ACCOUNT_STATE_OFFSET
    pub task_id: u32,                       //  4 bytes, WORKER_ACCOUNT_TASK_ID_OFFSET
    pub instr_id: u32,                      //  4 bytes, WORKER_ACCOUNT_INSTR_ID_OFFSET
    pub ops_counter: i64,                   //  8 bytes, WORKER_ACCOUNT_OPS_COUNTER_OFFSET
    pub first_time: i64,                    //  8 bytes, WORKER_ACCOUNT_FIRST_TIME_OFFSET
    pub duration: u32,                      //  4 bytes, WORKER_ACCOUNT_DURATION_OFFSET
    init_px: [u8; 8],                       //  8 bytes, WORKER_ACCOUNT_INIT_PX_OFFSET
    pub own_ps: u32,                        //  4 bytes, WORKER_ACCOUNT_OWN_PS_OFFSET
    pub width_factor: i64,                  //  8 bytes, WORKER_ACCOUNT_WIDTH_FACTOR_OFFSET
    pub fee_rate: i64,                      //  8 bytes, WORKER_ACCOUNT_FEE_RATE_OFFSET
    pub fee_ratio: i64,                     //  8 bytes, WORKER_ACCOUNT_FEE_RATIO_OFFSET
    pub inventories_ratio: i64,             //  8 bytes, WORKER_ACCOUNT_INVENTORIES_RATIO_OFFSET
    pub fractions: u32,                     //  4 bytes, WORKER_ACCOUNT_FRACTIONS_OFFSET
    max_order: [u8; 8],                     //  8 bytes, WORKER_ACCOUNT_MAX_ORDER_OFFSET
    new_width_factor: [u8; 8],              //  8 bytes, WORKER_ACCOUNT_NEW_WIDTH_FACTOR_OFFSET
    new_fee_rate: [u8; 8],                  //  8 bytes, WORKER_ACCOUNT_NEW_FEE_RATE_OFFSET
    new_fee_ratio: [u8; 8],                 //  8 bytes, WORKER_ACCOUNT_NEW_FEE_RATIO_OFFSET
    new_inventories_ratio: [u8; 8],         //  8 bytes, WORKER_ACCOUNT_NEW_INVENTORIES_RATIO_OFFSET
    pub new_fractions: u32,                 //  4 bytes, WORKER_ACCOUNT_NEW_FRACTIONS_OFFSET
    pub new_max_order: i64,                 //  8 bytes, WORKER_ACCOUNT_NEW_MAX_ORDER_OFFSET
    pub new_max_inventories: u64,           //  8 bytes, WORKER_ACCOUNT_NEW_MAX_INVENTORIES_OFFSET
    pub pool_time: u64,                     //  8 bytes, WORKER_ACCOUNT_POOL_TIME_OFFSET
    pub lp_counter: u64,                    //  8 bytes, WORKER_ACCOUNT_LP_COUNTER_OFFSET
    pub last_lp_time: u64,                  //  8 bytes, WORKER_ACCOUNT_LAST_LP_TIME_OFFSET
    pub pool_size: i64,                     //  8 bytes, WORKER_ACCOUNT_POOL_SIZE_OFFSET
    pub cps: u64,                           //  8 bytes, WORKER_ACCOUNT_CPS_OFFSET
    pub cps_px: u64,                        //  8 bytes, WORKER_ACCOUNT_CPS_PX_OFFSET
    pub cps_in_ps: u64,                     //  8 bytes, WORKER_ACCOUNT_CPS_IN_PS_OFFSET
    pub ps: u32,                            //  4 bytes, WORKER_ACCOUNT_PS_OFFSET
    ps_px: [u8; 8],                         //  8 bytes, WORKER_ACCOUNT_PS_PX_OFFSET
    task_fees: [u8; 8],                     //  8 bytes, WORKER_ACCOUNT_TASK_FEES_OFFSET
    rest_of_fees: [u8; 8],                  //  8 bytes, WORKER_ACCOUNT_REST_OF_FEES_OFFSET
    pub pool_id: u32,                       //  4 bytes, WORKER_ACCOUNT_POOL_ID_OFFSET
    pub start: i64,                         //  8 bytes, WORKER_ACCOUNT_START_OFFSET
    pub expiration: i64,                    //  8 bytes, WORKER_ACCOUNT_EXPIRATION_OFFSET
    pub pool_cashflow: u64,                 //  8 bytes, WORKER_ACCOUNT_POOL_CASHFLOW_OFFSET
    pub settlement_px: u64,                 //  8 bytes, WORKER_ACCOUNT_SETTLEMENT_PX_OFFSET
    pub pool_depo: i64,                     //  8 bytes, WORKER_ACCOUNT_POOL_DEPO_OFFSET
    pub pool_fees: u64,                     //  8 bytes, WORKER_ACCOUNT_POOL_FEES_OFFSET
    pub pool_counter: u64,                  //  8 bytes, WORKER_ACCOUNT_POOL_COUNTER_OFFSET
    pub pool_strikes: [i64; BUCKETS_COUNT], //  760 bytes, WORKER_ACCOUNT_POOL_STRIKES_OFFSET
    pub pool_bounds: [i64; BOUNDS_COUNT],   //  752 bytes, WORKER_ACCOUNT_POOL_BOUNDS_OFFSET
    pub pool_distrib: [Band; BUCKETS_COUNT],//  3040 bytes, WORKER_ACCOUNT_POOL_DISTRIB_OFFSET
    pub svm_params: SvmParams,              //  40 bytes, WORKER_ACCOUNT_SVM_PARAMS_OFFSET
    pub margin_vega: i64,                   //  8 bytes, WORKER_ACCOUNT_MARGIN_VEGA_OFFSET
    pub margin_vanna: i64,                  //  8 bytes, WORKER_ACCOUNT_MARGIN_VANNA_OFFSET
    pub margin_volga: i64,                  //  8 bytes, WORKER_ACCOUNT_MARGIN_VOLGA_OFFSET
    pub range_lr: i64,                      //  8 bytes, WORKER_ACCOUNT_RANGE_LR_OFFSET
    pub w_lr: i64,                          //  8 bytes, WORKER_ACCOUNT_W_LR_OFFSET
    pub max_lr: i64,                        //  8 bytes, WORKER_ACCOUNT_MAX_LR_OFFSET
    pub max_pct_pool: i64,                  //  8 bytes, WORKER_ACCOUNT_MAX_PCT_POOL_OFFSET
    pub perm_impact: f64,                   //  8 bytes, WORKER_PERM_IMPACT_OFFSET
}

impl WorkerAccount {
    #[inline(always)]
    pub fn get_init_px(&self) -> i64 { i64::from_ne_bytes(self.init_px) }

    #[inline(always)]
    pub fn set_init_px(&mut self, value: i64) { self.init_px = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_max_order(&self) -> i64 { i64::from_ne_bytes(self.max_order) }

    #[inline(always)]
    pub fn set_max_order(&mut self, value: i64) { self.max_order = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_new_width_factor(&self) -> i64 { i64::from_ne_bytes(self.new_width_factor) }

    #[inline(always)]
    pub fn set_new_width_factor(&mut self, value: i64) { self.new_width_factor = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_new_fee_rate(&self) -> i64 { i64::from_ne_bytes(self.new_fee_rate) }

    #[inline(always)]
    pub fn set_new_fee_rate(&mut self, value: i64) { self.new_fee_rate = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_new_fee_ratio(&self) -> i64 { i64::from_ne_bytes(self.new_fee_ratio) }

    #[inline(always)]
    pub fn set_new_fee_ratio(&mut self, value: i64) { self.new_fee_ratio = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_new_inventories_ratio(&self) -> i64 { i64::from_ne_bytes(self.new_inventories_ratio) }

    #[inline(always)]
    pub fn set_new_inventories_ratio(&mut self, value: i64) { self.new_inventories_ratio = value.to_ne_bytes() }

}

impl DevolIndexedAccount for WorkerAccount {}

impl DvlParametrable for WorkerAccount { type DvlReadParams<'a> = DvlIndexParam; }

impl DevolAccount for WorkerAccount {
    fn expected_size() -> usize {
        WORKER_ACCOUNT_SIZE
    }

    fn expected_tag() -> u8 {
        WORKER_ACCOUNT_TAG
    }

    fn expected_version() -> u32 {
       WORKER_ACCOUNT_VERSION
    }

    #[inline(always)]
    fn check_additional<'a>(_account_info: &AccountInfo, _params: &Self::DvlReadParams<'a>) -> Result<(), DvlError> {
        Self::check_id(_account_info, Some(_params.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    impl Band {
        pub fn new(depo: i64, px: i64, loan: i64, prop: i64) -> Self {
            Band { depo, px, loan, prop }
        }
    }

    #[test]
    fn test_worker_account_offsets() {
        let band_array: [Band; BUCKETS_COUNT] = [Band::new(0, 0, 0, 0); BUCKETS_COUNT];
        let bounds_array: [i64; BOUNDS_COUNT] = [0; BOUNDS_COUNT];
        let strikes_array: [i64; BUCKETS_COUNT] = [0; BUCKETS_COUNT];

        let account = WorkerAccount {
            header: AccountHeader::default(),
            id: 0,
            state: WorkerState::Unassigned,
            task_id: 0,
            instr_id: 0,
            ops_counter: 0,
            first_time: 0,
            duration: 0,
            init_px: [0; 8],
            own_ps: 0,
            width_factor: 0,
            fee_rate: 0,
            fee_ratio: 0,
            inventories_ratio: 0,
            fractions: 0,
            max_order: [0; 8],
            new_width_factor: [0; 8],
            new_fee_rate: [0; 8],
            new_fee_ratio: [0; 8],
            new_inventories_ratio: [0; 8],
            new_fractions: 0,
            new_max_order: 0,
            new_max_inventories: 0,
            pool_time: 0,
            lp_counter: 0,
            last_lp_time: 0,
            pool_size: 0,
            cps: 0,
            cps_px: 0,
            cps_in_ps: 0,
            ps: 0,
            ps_px: [0; 8],
            task_fees: [0; 8],
            rest_of_fees: [0; 8],
            pool_id: 0,
            start: 0,
            expiration: 0,
            pool_cashflow: 0,
            settlement_px: 0,
            pool_depo: 0,
            pool_fees: 0,
            pool_counter: 0,
            pool_strikes: strikes_array,
            pool_bounds: bounds_array,
            pool_distrib: band_array,
            svm_params: SvmParams { psi: 0.0, p: 0.0, c: 0.0, vt: 0.0, v: 0.0 },
            margin_vega: 0,
            margin_vanna: 0,
            margin_volga: 0,
            max_lr: 0,
            max_pct_pool: 0,
            range_lr: 0,
            w_lr: 0,
            perm_impact: 0.0,
        };

        let base_ptr = &account as *const _ as usize;
        // checking fields size
        assert_eq!(&account.header.tag as *const _ as usize - base_ptr, WORKER_ACCOUNT_TAG_OFFSET);
        assert_eq!(&account.header.version as *const _ as usize - base_ptr, WORKER_ACCOUNT_VERSION_OFFSET);
        assert_eq!(&account.header.root as *const _ as usize - base_ptr, WORKER_ACCOUNT_ROOT_ADDRESS_OFFSET);
        assert_eq!(&account.id as *const _ as usize - base_ptr, WORKER_ACCOUNT_ID_OFFSET);
        assert_eq!(&account.state as *const _ as usize - base_ptr, WORKER_ACCOUNT_STATE_OFFSET);
        assert_eq!(&account.task_id as *const _ as usize - base_ptr, WORKER_ACCOUNT_TASK_ID_OFFSET);
        assert_eq!(&account.instr_id as *const _ as usize - base_ptr, WORKER_ACCOUNT_INSTR_ID_OFFSET);
        assert_eq!(&account.ops_counter as *const _ as usize - base_ptr, WORKER_ACCOUNT_OPS_COUNTER_OFFSET);
        assert_eq!(&account.first_time as *const _ as usize - base_ptr, WORKER_ACCOUNT_FIRST_TIME_OFFSET);
        assert_eq!(&account.duration as *const _ as usize - base_ptr, WORKER_ACCOUNT_DURATION_OFFSET);
        assert_eq!(&account.init_px as *const _ as usize - base_ptr, WORKER_ACCOUNT_INIT_PX_OFFSET);
        assert_eq!(&account.own_ps as *const _ as usize - base_ptr, WORKER_ACCOUNT_OWN_PS_OFFSET);
        assert_eq!(&account.width_factor as *const _ as usize - base_ptr, WORKER_ACCOUNT_WIDTH_FACTOR_OFFSET);
        assert_eq!(&account.fee_rate as *const _ as usize - base_ptr, WORKER_ACCOUNT_FEE_RATE_OFFSET);
        assert_eq!(&account.fee_ratio as *const _ as usize - base_ptr, WORKER_ACCOUNT_FEE_RATIO_OFFSET);
        assert_eq!(&account.inventories_ratio as *const _ as usize - base_ptr, WORKER_ACCOUNT_INVENTORIES_RATIO_OFFSET);
        assert_eq!(&account.fractions as *const _ as usize - base_ptr, WORKER_ACCOUNT_FRACTIONS_OFFSET);
        assert_eq!(&account.max_order as *const _ as usize - base_ptr, WORKER_ACCOUNT_MAX_ORDER_OFFSET);
        assert_eq!(&account.new_width_factor as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_WIDTH_FACTOR_OFFSET);
        assert_eq!(&account.new_fee_rate as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_FEE_RATE_OFFSET);
        assert_eq!(&account.new_fee_ratio as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_FEE_RATIO_OFFSET);
        assert_eq!(&account.new_inventories_ratio as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_INVENTORIES_RATIO_OFFSET);
        assert_eq!(&account.new_fractions as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_FRACTIONS_OFFSET);
        assert_eq!(&account.new_max_order as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_MAX_ORDER_OFFSET);
        assert_eq!(&account.new_max_inventories as *const _ as usize - base_ptr, WORKER_ACCOUNT_NEW_MAX_INVENTORIES_OFFSET);
        assert_eq!(&account.pool_time as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_TIME_OFFSET);
        assert_eq!(&account.lp_counter as *const _ as usize - base_ptr, WORKER_ACCOUNT_LP_COUNTER_OFFSET);
        assert_eq!(&account.last_lp_time as *const _ as usize - base_ptr, WORKER_ACCOUNT_LAST_LP_TIME_OFFSET);
        assert_eq!(&account.pool_size as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_SIZE_OFFSET);
        assert_eq!(&account.cps as *const _ as usize - base_ptr, WORKER_ACCOUNT_CPS_OFFSET);
        assert_eq!(&account.cps_px as *const _ as usize - base_ptr, WORKER_ACCOUNT_CPS_PX_OFFSET);
        assert_eq!(&account.cps_in_ps as *const _ as usize - base_ptr, WORKER_ACCOUNT_CPS_IN_PS_OFFSET);
        assert_eq!(&account.ps as *const _ as usize - base_ptr, WORKER_ACCOUNT_PS_OFFSET);
        assert_eq!(&account.ps_px as *const _ as usize - base_ptr, WORKER_ACCOUNT_PS_PX_OFFSET);
        assert_eq!(&account.task_fees as *const _ as usize - base_ptr, WORKER_ACCOUNT_TASK_FEES_OFFSET);
        assert_eq!(&account.rest_of_fees as *const _ as usize - base_ptr, WORKER_ACCOUNT_REST_OF_FEES_OFFSET);
        assert_eq!(&account.pool_id as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_ID_OFFSET);
        assert_eq!(&account.start as *const _ as usize - base_ptr, WORKER_ACCOUNT_START_OFFSET);
        assert_eq!(&account.expiration as *const _ as usize - base_ptr, WORKER_ACCOUNT_EXPIRATION_OFFSET);
        assert_eq!(&account.pool_cashflow as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_CASHFLOW_OFFSET);
        assert_eq!(&account.settlement_px as *const _ as usize - base_ptr, WORKER_ACCOUNT_SETTLEMENT_PX_OFFSET);
        assert_eq!(&account.pool_depo as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_DEPO_OFFSET);
        assert_eq!(&account.pool_fees as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_FEES_OFFSET);
        assert_eq!(&account.pool_counter as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_COUNTER_OFFSET);
        assert_eq!(&account.pool_strikes as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_STRIKES_OFFSET);
        assert_eq!(&account.pool_bounds as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_BOUNDS_OFFSET);
        assert_eq!(&account.pool_distrib as *const _ as usize - base_ptr, WORKER_ACCOUNT_POOL_DISTRIB_OFFSET);
        assert_eq!(&account.svm_params as *const _ as usize - base_ptr, WORKER_ACCOUNT_SVM_PARAMS_OFFSET);
        assert_eq!(&account.margin_vega as *const _ as usize - base_ptr, WORKER_ACCOUNT_MARGIN_VEGA_OFFSET);
        assert_eq!(&account.margin_vanna as *const _ as usize - base_ptr, WORKER_ACCOUNT_MARGIN_VANNA_OFFSET);
        assert_eq!(&account.margin_volga as *const _ as usize - base_ptr, WORKER_ACCOUNT_MARGIN_VOLGA_OFFSET);
        assert_eq!(&account.range_lr as *const _ as usize - base_ptr, WORKER_ACCOUNT_RANGE_LR_OFFSET);
        assert_eq!(&account.w_lr as *const _ as usize - base_ptr, WORKER_ACCOUNT_W_LR_OFFSET);
        assert_eq!(&account.max_lr as *const _ as usize - base_ptr, WORKER_ACCOUNT_MAX_LR_OFFSET);
        assert_eq!(&account.max_pct_pool as *const _ as usize - base_ptr, WORKER_ACCOUNT_MAX_PCT_POOL_OFFSET);
        assert_eq!(&account.perm_impact as *const _ as usize - base_ptr, WORKER_PERM_IMPACT_OFFSET);

        // checking total size
        assert_eq!(mem::size_of::<WorkerAccount>(), WORKER_ACCOUNT_SIZE);
    }
}
