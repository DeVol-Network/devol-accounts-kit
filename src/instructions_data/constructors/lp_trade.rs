use std::error::Error;
use crate::instructions_data::dvl_instruction_data::DvlInstructionData;
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::lp_trade::{INSTRUCTION_LP_TRADE_VERSION, InstructionLpTrade};

pub struct LpTradeParams {
    pub trade_qty: i32,
}

impl<'a> DvlInstructionData<'a> for InstructionLpTrade {
    type DvlInstrParams = LpTradeParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionLpTrade>, Box<dyn Error>> {
        Ok(Box::new(InstructionLpTrade {
            cmd: Instructions::LpTrade as u8,
            version: INSTRUCTION_LP_TRADE_VERSION,
            reserved: [0; 2],
            trade_qty: params.trade_qty,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions_data::dvl_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_lp_trade_params() {
        const TEST_TRADE_QTY: i32 = 20;

        let lp_trade_params = LpTradeParams {
            trade_qty: TEST_TRADE_QTY
        };
        let data = DvlInstruction::new::<InstructionLpTrade>(lp_trade_params).unwrap();
        assert_eq!(data.cmd, Instructions::LpTrade as u8);
        assert_eq!(data.version, INSTRUCTION_LP_TRADE_VERSION);
        assert_eq!(data.trade_qty, TEST_TRADE_QTY);
    }
}