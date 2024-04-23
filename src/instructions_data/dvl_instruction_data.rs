use std::error::Error;
use crate::transactions_instructions::as_transaction_instruction::AsTransactionInstruction;

pub trait DvlInstructionData<'a> : AsTransactionInstruction {
    type DvlInstrParams: 'a;
    fn new(params: Self::DvlInstrParams) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized;

    fn as_vec_le(&self) -> Vec<u8> where Self: Sized {
        let data_bytes = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>())
        };
        data_bytes.to_vec()
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