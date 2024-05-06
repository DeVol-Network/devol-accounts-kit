use std::error::Error;
use crate::accounts::worker::svm_params::SvmParams;
use crate::constants::{BOUNDS_COUNT, BUCKETS_COUNT, FD};
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::start_next_pool::{INSTRUCTION_START_NEXT_POOL_VERSION, InstructionStartNextPool};

pub struct StartNextPoolParams {
    pub svm_params: SvmParams,
    pub prices: [f32; BUCKETS_COUNT],
    pub bounds: [f32; BOUNDS_COUNT],
    pub margin_vega: f64,
    pub margin_vanna: f64,
    pub margin_volga: f64,
    pub range_lr: f64,
    pub w_lr: f64,
    pub max_lr: f64,
    pub max_pct_pool: f64,
    pub perm_impact: f64,
}

impl<'a> DvlInstructionData<'a> for InstructionStartNextPool {
    type DvlInstrParams = StartNextPoolParams;

    fn new(
        params: Self::DvlInstrParams,
    ) -> Result<Box<InstructionStartNextPool>, Box<dyn Error>> {
        let mut bounds_i32 = [0; BOUNDS_COUNT];
        for i in 0..BOUNDS_COUNT {
            bounds_i32[i] = (params.bounds[i] + 0.5) as i32;
        }
        Ok(Box::new(InstructionStartNextPool {
            cmd: Instructions::StartNextPool as u8,
            version: INSTRUCTION_START_NEXT_POOL_VERSION,
            reserved: [0; 6],
            prices: params.prices,
            svm_params: params.svm_params,
            margin_vega: (params.margin_vega * FD) as i64,
            margin_vanna: (params.margin_vanna * FD) as i64,
            margin_volga: (params.margin_volga * FD) as i64,
            bounds: bounds_i32,
            reserved2: [0; 4],
            w_lr: (params.w_lr * FD) as i64,
            range_lr: (params.range_lr * FD) as i64,
            max_pct_pool: (params.max_pct_pool * FD) as i64,
            max_lr: (params.max_lr * FD) as i64,
            perm_impact: params.perm_impact,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_start_next_pool_params() {
        const TEST_PRICE: f32 = 10.0;
        const TEST_C: f64 = 10.0;
        const TEST_P: f64 = 20.0;
        const TEST_V: f64 = 30.0;
        const TEST_VT: f64 = 40.0;
        const TEST_PSI: f64 = 20.0;
        const TEST_MARGIN: f64 = 20.;
        const TEST_BOUND: i32 = 30;
        const TEST_LR: f64 = 10.;
        const TEST_IMPACT: f64 = 5.0;

        let start_next_pool_params = StartNextPoolParams {
            prices: [TEST_PRICE; BUCKETS_COUNT],
            svm_params: SvmParams {
                c: TEST_C,
                p: TEST_P,
                v: TEST_V,
                vt: TEST_VT,
                psi: TEST_PSI,
            },
            margin_vega: TEST_MARGIN,
            margin_vanna: TEST_MARGIN,
            margin_volga: TEST_MARGIN,
            bounds: [TEST_BOUND as f32; BOUNDS_COUNT],
            w_lr: TEST_LR,
            range_lr: TEST_LR,
            max_pct_pool: TEST_LR,
            max_lr: TEST_LR,
            perm_impact: TEST_IMPACT,
        };
        let data =
            DvlInstruction::new::<InstructionStartNextPool>(start_next_pool_params).unwrap();
        assert_eq!(data.cmd, Instructions::StartNextPool as u8);
        assert_eq!(data.version, INSTRUCTION_START_NEXT_POOL_VERSION);
        assert_eq!(data.prices, [TEST_PRICE; BUCKETS_COUNT]);
        assert_eq!(data.svm_params.c, TEST_C);
        assert_eq!(data.svm_params.p, TEST_P);
        assert_eq!(data.svm_params.v, TEST_V);
        assert_eq!(data.svm_params.vt, TEST_VT);
        assert_eq!(data.svm_params.psi, TEST_PSI);
        assert_eq!(data.margin_vega as f64 / FD, TEST_MARGIN);
        assert_eq!(data.margin_vanna as f64 / FD, TEST_MARGIN);
        assert_eq!(data.margin_volga as f64 / FD, TEST_MARGIN);
        assert_eq!(data.bounds, [TEST_BOUND; BOUNDS_COUNT]);
        assert_eq!(data.w_lr as f64 / FD, TEST_LR);
        assert_eq!(data.range_lr as f64 / FD, TEST_LR);
        assert_eq!(data.max_pct_pool as f64 / FD, TEST_LR);
        assert_eq!(data.max_lr as f64 / FD, TEST_LR);
        assert_eq!(data.perm_impact, TEST_IMPACT);
    }
}
