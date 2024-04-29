use std::error::Error;
use crate::dvl_error::DvlError;
use crate::errors::ContractError;
cfg_if::cfg_if! {
    if #[cfg(not(feature = "on-chain"))] {
        use crate::instructions_data::as_transaction_instruction::as_transaction_instruction::AsTransactionInstruction;
    }
    else {
        use crate::instructions_data::as_transaction_instruction_on_chain::AsTransactionInstruction;
    }
}


pub trait DvlInstructionData<'a> : AsTransactionInstruction {
    fn expected_size() -> usize;
    fn expected_version() -> u8;
    #[inline(always)]
    fn check_version(vec: &'a[u8]) -> Result<(), DvlError> {
        if vec[1] != Self::expected_version() {
            return Err(DvlError::new(ContractError::InstructionDataVersion));
        }
        Ok(())
    }

    type DvlInstrParams: 'a;
    fn new(params: Self::DvlInstrParams) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized;

    fn to_vec_le(&self) -> Vec<u8> where Self: Sized {
        let data_bytes = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>())
        };
        data_bytes.to_vec()
    }

    fn from_vec_le(vec: &'a[u8]) -> Result<&'a Self, DvlError> where Self: Sized {
        if vec.len() != Self::expected_size() {
            return Err(DvlError::new(ContractError::InstructionDataLength));
        }
        Ok(unsafe { &*(vec.as_ptr() as *const Self) })
    }
}

pub struct DvlInstruction;

impl DvlInstruction {
    pub fn new<'a, T>(params: T::DvlInstrParams) -> Result<Box<T>, Box<dyn Error>>
        where
            T: DvlInstructionData<'a>
    {
        T::new(params)
    }
}