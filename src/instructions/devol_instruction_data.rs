use std::error::Error;

pub trait DevolInstructionData<'a> {
    type DvlInstrParams :'a;
    fn new(params: Self::DvlInstrParams) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized;
}

pub struct DvlInstruction;

impl DvlInstruction {
    pub fn new<'a, T>(params: T::DvlInstrParams) -> Result<Box<T>, Box<dyn Error>>
        where
            T: DevolInstructionData<'a>
    {
        T::new(params)
    }
}