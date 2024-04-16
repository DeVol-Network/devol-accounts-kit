use crate::instructions::option_trade::{InstructionOptionTrade};

pub const INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE: usize = 456;

#[repr(C)]
pub struct InstructionOptionTradeDebug {
    option_trade: InstructionOptionTrade,
    time_to_expiration: i64,
    underlying_price: i64,
}

const INSTRUCTION_VERSION: u8 = 2;

#[cfg(test)]
impl Default for InstructionOptionTradeDebug{
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

    #[test]
    fn test_instruction_data_offsets() {
        let data = InstructionOptionTradeDebug::default();
        let base_ptr = &data as *const _ as usize;

        assert_eq!(&data.option_trade as *const _ as usize  - base_ptr, 0);
        assert_eq!(&data.time_to_expiration as *const _ as usize  - base_ptr, 440);
        assert_eq!(&data.underlying_price as *const _ as usize  - base_ptr, 448);

        assert_eq!(mem::size_of::<InstructionOptionTradeDebug>(), INSTRUCTION_OPTION_TRADE_DEBUG_DATA_SIZE);
    }
}