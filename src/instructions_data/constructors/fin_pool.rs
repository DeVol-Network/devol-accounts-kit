use std::error::Error;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::fin_pool::{INSTRUCTION_FIN_POOL_VERSION, InstructionFinPool};
use crate::instructions_data::instructions::Instructions;

pub struct FinPoolParams {
    pub price: i64,
    pub terminate: bool,
}

impl<'a> DvlInstructionData<'a> for InstructionFinPool {
    type DvlInstrParams = FinPoolParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionFinPool>, Box<dyn Error>> {
        Ok(Box::new(InstructionFinPool {
            cmd: Instructions::FinPool as u8,
            version: INSTRUCTION_FIN_POOL_VERSION,
            reserved: [0; 5],
            price: params.price,
            terminate: params.terminate,
        }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::constructors::fin_pool::FinPoolParams;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_fin_pool_params() {
        const TEST_PRICE: i64 = 10;
        const TEST_TERMINATE: bool = true;

        let fin_pool_params = FinPoolParams {
            price: TEST_PRICE,
            terminate: TEST_TERMINATE,
        };
        let data = DvlInstruction::new::<InstructionFinPool>(fin_pool_params).unwrap();
        assert_eq!(data.cmd, Instructions::FinPool as u8);
        assert_eq!(data.version, INSTRUCTION_FIN_POOL_VERSION);
        assert_eq!(data.price, TEST_PRICE);
        assert_eq!(data.terminate, TEST_TERMINATE);
    }
}