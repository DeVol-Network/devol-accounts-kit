use std::error::Error;
use crate::constants::BUCKETS_COUNT;
use crate::instructions::devol_instruction_data::{DevolInstructionData, DvlInstruction};
use crate::instructions::instructions::Instructions;
use crate::instructions::option_trade::{BasketData, InstructionOptionTrade, OptionTradeParams};

#[repr(C)]
pub struct InstructionOptionTradeDebug {
    pub option_trade: InstructionOptionTrade,
    pub time_to_expiration: i64,
    pub underlying_price: i64,
}

pub struct OptionTradeDebugParams<'a> {
    pub trade_qty: [i32; BUCKETS_COUNT],
    pub max_cost: Option<i64>,
    pub basket: Option<&'a [BasketData]>,
    pub time_to_expiration: i64,
    pub underlying_price: i64,
}

impl<'a> DevolInstructionData<'a> for InstructionOptionTradeDebug {
    type DvlInstrParams = OptionTradeDebugParams<'a>;

    fn new(params: Self::DvlInstrParams) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized {
        let trade_params = OptionTradeParams {
            basket: params.basket,
            max_cost: params.max_cost,
            trade_qty: params.trade_qty,
        };
        let mut option_trade = DvlInstruction::new::<InstructionOptionTrade>(trade_params)?;
        option_trade.cmd = Instructions::OptionTradeDebug as u8;
        Ok(Box::new(InstructionOptionTradeDebug {
            option_trade: *option_trade,
            underlying_price: params.underlying_price,
            time_to_expiration: params.time_to_expiration,
        }))
    }
}

#[cfg(test)]
impl Default for InstructionOptionTradeDebug {
    fn default() -> Self {
        InstructionOptionTradeDebug {
            option_trade: InstructionOptionTrade::default(),
            time_to_expiration: 0,
            underlying_price: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    pub const INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE: usize = 456;
    pub const INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET: usize = 0;
    pub const INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET: usize = 440;
    pub const INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET: usize = 448;

    #[test]
    fn test_instruction_data_offsets() {
        let data = InstructionOptionTradeDebug::default();
        let base_ptr = &data as *const _ as usize;

        assert_eq!(&data.option_trade as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_OPTION_TRADE_OFFSET);
        assert_eq!(&data.time_to_expiration as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_TIME_TO_EXPIRATION_OFFSET);
        assert_eq!(&data.underlying_price as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_DEBUG_UNDERLYING_PRICE_OFFSET);

        assert_eq!(mem::size_of::<InstructionOptionTradeDebug>(), INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE);
    }

    #[test]
    fn test_default_instruction_option_trade_debug() {
        let debug_trade = InstructionOptionTradeDebug::default();

        assert_eq!(debug_trade.time_to_expiration, 0, "The default value for time_to_expiration should be zero.");
        assert_eq!(debug_trade.underlying_price, 0, "The default value for underlying_price should be zero.");
    }

    #[test]
    fn test_specific_instruction_option_trade_debug() {
        let test_max_cost = 200;
        let test_trade_qty = [100; BUCKETS_COUNT];
        let test_time_to_expiration = 3600;
        let test_underlying_price = 50000;

        let params = OptionTradeDebugParams {
            trade_qty: test_trade_qty,
            max_cost: Some(test_max_cost),
            basket: None,
            time_to_expiration: test_time_to_expiration,
            underlying_price: test_underlying_price,
        };

        let debug_trade_result = InstructionOptionTradeDebug::new(params);
        assert!(debug_trade_result.is_ok());

        let debug_trade = debug_trade_result.unwrap();
        assert_eq!(debug_trade.time_to_expiration, test_time_to_expiration);
        assert_eq!(debug_trade.underlying_price, test_underlying_price);
        assert_eq!(debug_trade.option_trade.cmd, Instructions::OptionTradeDebug as u8);
        assert_eq!(debug_trade.option_trade.max_cost, test_max_cost);
        assert_eq!(debug_trade.option_trade.trade_qty, test_trade_qty);
    }

}
