use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::fin_pool::InstructionFinPool;
use crate::instructions::instructions::Instructions;

pub struct FinPoolParams {
    pub price: i64,
    pub terminate: bool,
}

impl<'a> DevolInstructionData<'a> for InstructionFinPool {
    type DvlInstrParams = FinPoolParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionFinPool>, Box<dyn Error>> {
        Ok(Box::new(InstructionFinPool {
            cmd: Instructions::FinPool as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 5],
            price: params.price,
            terminate: params.terminate,
        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::constructors::fin_pool::FinPoolParams;
    use crate::instructions::devol_instruction_data::DvlInstruction;

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
        assert_eq!(data.version, INSTRUCTION_VERSION);
        assert_eq!(data.price, TEST_PRICE);
        assert_eq!(data.terminate, TEST_TERMINATE);
    }
}