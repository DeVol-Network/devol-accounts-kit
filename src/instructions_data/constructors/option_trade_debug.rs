use std::error::Error;
use crate::instructions_data::constructors::option_trade::OptionTradeParams;
use crate::instructions_data::dvl_instruction_data::{DvlInstructionData, DvlInstruction};
use crate::instructions_data::instructions::Instructions;
use crate::instructions_data::option_trade::InstructionOptionTrade;
use crate::instructions_data::option_trade_debug::{InstructionOptionTradeDebug, OptionTradeDebugParams};

impl<'a> DvlInstructionData<'a> for InstructionOptionTradeDebug {
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
    use crate::constants::BUCKETS_COUNT;

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