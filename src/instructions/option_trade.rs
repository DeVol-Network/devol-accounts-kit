use std::error::Error;
use crate::constants::BUCKETS_COUNT;
use crate::dvl_off_chain_error::DvlOffChainError;
use crate::instructions::devol_instruction_data::DevolInstructionData;

#[derive(Copy, Clone)]
#[repr(C)]
struct BasketData {
    strike: u32,      //  4 bytes, offset: 0
    pc: u32,          //  4 bytes, offset: 4
    amount: i32,      //  4 bytes, offset: 8
}

pub const INSTRUCTION_OPTION_TRADE_DATA_SIZE: usize = 440;

#[repr(C)]
pub struct InstructionOptionTrade {
    cmd: u8,
    version: u8,
    reserved: u8,
    basket_length: u8,
    trade_qty: [i32; BUCKETS_COUNT],
    max_cost: i64,
    basket: [BasketData; INSTR_OPTION_TRADE_MAX_BASKET_LENGTH],
}

pub struct OptionTradeParams<'a> {
    trade_qty: [i32; BUCKETS_COUNT],
    max_cost: Option<i64>,
    basket: Option<&'a[BasketData]>,
}

impl<'b> DevolInstructionData<'b> for InstructionOptionTrade {
    type DvlInstrParams = OptionTradeParams<'b>;

    fn new(params: Self::DvlInstrParams) -> Result<Self, Box<dyn Error>> {
        let basket_length = params.basket.map_or(0, |basket| basket.len());
        if basket_length > 4 {
            Err(DvlOffChainError::BasketTooLarge)?;
        }

        Ok(InstructionOptionTrade{
            cmd: 112,
            version: INSTRUCTION_VERSION,
            reserved: 0,
            basket_length: basket_length as u8,
            trade_qty: params.trade_qty,
            max_cost: params.max_cost.unwrap_or(-1000000000),
            basket: <[BasketData; 4]>::try_from(params.basket.unwrap()).unwrap(),
        })
    }
}

pub const INSTR_OPTION_TRADE_CMD_OFFSET: usize = 0;
pub const INSTR_OPTION_TRADE_VERSION_OFFSET: usize = 1;
pub const INSTR_OPTION_TRADE_RESERVED_OFFSET: usize = 2;
pub const INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET: usize = 3;
pub const INSTR_OPTION_TRADE_TRADE_QTY_OFFSET: usize = 4;
pub const INSTR_OPTION_TRADE_MAX_COST_OFFSET: usize = 384;
pub const INSTR_OPTION_TRADE_BASKET_DATA_OFFSET: usize = 392;
pub const INSTR_OPTION_TRADE_MAX_BASKET_LENGTH: usize = 4;
const INSTRUCTION_VERSION: u8 = 2;

#[cfg(test)]
impl Default for InstructionOptionTrade{
    fn default() -> Self {
        InstructionOptionTrade{
            cmd: 0,
            basket_length: 0,
            version: 0,
            reserved: 0,
            trade_qty: [0; BUCKETS_COUNT],
            max_cost: 0,
            basket: [BasketData { strike: 0, pc: 0, amount: 0 }; 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_instruction_data_offsets() {
        assert_eq!(mem::size_of::<u8>(), 1);
        assert_eq!(mem::size_of::<i32>(), 4);
        assert_eq!(mem::size_of::<i64>(), 8);
        assert_eq!(mem::size_of::<BasketData>(), 12);

        let data = InstructionOptionTrade::default();

        let base_ptr = &data as *const _ as usize;

        assert_eq!(&data.cmd as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_CMD_OFFSET);
        assert_eq!(&data.version as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_VERSION_OFFSET);
        assert_eq!(&data.reserved as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_RESERVED_OFFSET);
        assert_eq!(&data.basket_length as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_BASKET_LENGTH_OFFSET);
        assert_eq!(&data.trade_qty as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_TRADE_QTY_OFFSET);
        assert_eq!(&data.max_cost as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_MAX_COST_OFFSET);
        assert_eq!(&data.basket as *const _ as usize - base_ptr, INSTR_OPTION_TRADE_BASKET_DATA_OFFSET);

        assert_eq!(mem::size_of::<InstructionOptionTrade>(), INSTRUCTION_OPTION_TRADE_DATA_SIZE);
    }
}