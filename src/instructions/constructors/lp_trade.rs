use std::error::Error;
use crate::instructions::devol_instruction_data::DevolInstructionData;
use crate::instructions::instructions::Instructions;
use crate::instructions::lp_trade::{InstructionLpTrade};

pub struct LpTradeParams {
    pub trade_qty: i32,
}

impl<'a> DevolInstructionData<'a> for InstructionLpTrade {
    type DvlInstrParams = LpTradeParams;

    fn new(params: Self::DvlInstrParams) -> Result<Box<InstructionLpTrade>, Box<dyn Error>> {
        Ok(Box::new(InstructionLpTrade {
            cmd: Instructions::LpTrade as u8,
            version: INSTRUCTION_VERSION,
            reserved: [0; 2],
            trade_qty: params.trade_qty,
        }))
    }
}

const INSTRUCTION_VERSION: u8 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::devol_instruction_data::DvlInstruction;

    #[test]
    fn test_instruction_lp_trade_params() {
        const TEST_TRADE_QTY: i32 = 20;

        let lp_trade_params = LpTradeParams {
            trade_qty: TEST_TRADE_QTY
        };
        let data = DvlInstruction::new::<InstructionLpTrade>(lp_trade_params).unwrap();
        assert_eq!(data.cmd, Instructions::LpTrade as u8);
        assert_eq!(data.version, INSTRUCTION_VERSION);
        assert_eq!(data.trade_qty, TEST_TRADE_QTY);
    }
}