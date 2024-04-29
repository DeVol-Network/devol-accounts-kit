use std::error::Error;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::payoff::InstructionPayoff;

pub struct PayoffParams {
    pub worker_id: u32,
    pub pool_id: u32,
}

impl<'a> DvlInstructionData<'a> for InstructionPayoff {
    type DvlInstrParams = PayoffParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionPayoff>, Box<dyn Error>> {
        Ok(Box::new(InstructionPayoff {
            cmd: Instructions::Payoff as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 2],
            worker_id: params.worker_id,
            pool_id: params.pool_id,
        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_lp_trade_params() {
        const TEST_WORKER_ID: u32 = 1;
        const TEST_POOL_ID: u32 = 1;

        let payoff_params = PayoffParams {
            worker_id: TEST_WORKER_ID,
            pool_id: TEST_POOL_ID,
        };
        let data = DvlInstruction::new::<InstructionPayoff>(payoff_params).unwrap();
        assert_eq!(data.cmd, Instructions::Payoff as u8);
        assert_eq!(data.version, INSTRUCTION_VERSION);
        assert_eq!(data.worker_id, TEST_WORKER_ID);
        assert_eq!(data.pool_id, TEST_POOL_ID);
    }
}