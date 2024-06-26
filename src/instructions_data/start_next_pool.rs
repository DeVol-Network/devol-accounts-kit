use crate::accounts::worker::svm_params::SvmParams;
use crate::constants::{BOUNDS_COUNT};
use crate::instructions_data::dvl_deserializable_instruction::DvlDeserializableInstruction;

pub const INSTRUCTION_START_NEXT_POOL_SIZE: usize = 488;
pub const INSTRUCTION_START_NEXT_POOL_VERSION: u8 = 2;

#[repr(C)]
pub struct InstructionStartNextPool {
    pub cmd: u8,
    pub version: u8,
    pub reserved: [u8; 6],
    pub svm_params: SvmParams,
    pub bounds: [i32; BOUNDS_COUNT],
    pub margin_vega: i64,
    pub margin_vanna: i64,
    pub margin_volga: i64,
    pub range_lr: i64,
    pub w_lr: i64,
    pub max_lr: i64,
    pub max_pct_pool: i64,
    pub perm_impact: f64,
}

impl<'a> DvlDeserializableInstruction<'a> for InstructionStartNextPool {
    #[inline(always)]
    fn expected_size() -> usize {INSTRUCTION_START_NEXT_POOL_SIZE}
    #[inline(always)]
    fn expected_version() -> u8 {INSTRUCTION_START_NEXT_POOL_VERSION}
}

#[cfg(not(feature = "on-chain"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;
    use std::mem;
    use crate::instructions_data::constructors::start_next_pool::StartNextPoolParams;

    pub const INSTR_START_NEXT_POOL_CMD_OFFSET: usize = 0;
    pub const INSTR_START_NEXT_POOL_VERSION_OFFSET: usize = 1;
    pub const INSTR_START_NEXT_POOL_RESERVED_OFFSET: usize = 2;
    pub const INSTR_START_NEXT_POOL_SVM_PARAMS_OFFSET: usize = 8;
    pub const INSTR_START_NEXT_POOL_BOUNDS_OFFSET: usize = 48;
    pub const INSTR_START_NEXT_POOL_MARGIN_VEGA_OFFSET: usize = 424;
    pub const INSTR_START_NEXT_POOL_MARGIN_VANNA_OFFSET: usize = 432;
    pub const INSTR_START_NEXT_POOL_MARGIN_VOLGA_OFFSET: usize = 440;
    pub const INSTR_START_NEXT_POOL_RANGE_LR_OFFSET: usize = 448;
    pub const INSTR_START_NEXT_POOL_W_LR_OFFSET: usize = 456;
    pub const INSTR_START_NEXT_POOL_MAX_LR_OFFSET: usize = 464;
    pub const INSTR_START_NEXT_POOL_MAX_PCT_POOL_OFFSET: usize = 472;
    pub const INSTR_START_NEXT_POOL_PERM_IMPACT_OFFSET: usize = 480;

    #[test]
    fn test_instruction_data_offsets() {
        let start_next_pool_params = StartNextPoolParams {
            svm_params: SvmParams { c: 0., p: 0., v: 0., vt: 0., psi: 0. },
            margin_vega: 0.,
            margin_vanna: 0.,
            margin_volga: 0.,
            bounds: [0.; BOUNDS_COUNT],
            w_lr: 0.,
            range_lr: 0.,
            max_pct_pool: 0.,
            max_lr: 0.,
            perm_impact: 0.,
        };

        let data =
            DvlInstruction::new::<InstructionStartNextPool>(start_next_pool_params).unwrap();

        let base_ptr = &*data as *const _ as usize;

        assert_eq!(
            &data.cmd as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_CMD_OFFSET
        );
        assert_eq!(
            &data.version as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_VERSION_OFFSET
        );
        assert_eq!(
            &data.reserved as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_RESERVED_OFFSET
        );
        assert_eq!(
            &data.svm_params as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_SVM_PARAMS_OFFSET
        );
        assert_eq!(
            &data.bounds as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_BOUNDS_OFFSET
        );
        assert_eq!(
            &data.margin_vega as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_MARGIN_VEGA_OFFSET
        );
        assert_eq!(
            &data.margin_vanna as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_MARGIN_VANNA_OFFSET
        );
        assert_eq!(
            &data.margin_volga as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_MARGIN_VOLGA_OFFSET
        );
        assert_eq!(
            &data.range_lr as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_RANGE_LR_OFFSET
        );
        assert_eq!(
            &data.w_lr as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_W_LR_OFFSET
        );
        assert_eq!(
            &data.max_lr as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_MAX_LR_OFFSET
        );
        assert_eq!(
            &data.max_pct_pool as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_MAX_PCT_POOL_OFFSET
        );
        assert_eq!(
            &data.perm_impact as *const _ as usize - base_ptr,
            INSTR_START_NEXT_POOL_PERM_IMPACT_OFFSET
        );

        assert_eq!(
            mem::size_of::<InstructionStartNextPool>(),
            INSTRUCTION_START_NEXT_POOL_SIZE
        );
    }
}
