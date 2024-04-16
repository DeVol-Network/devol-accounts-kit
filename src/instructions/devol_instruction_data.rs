use std::error::Error;

pub trait DevolInstructionData<'a> {
    type DvlInstrParams;
    fn new(_: Self::DvlInstrParams) -> Result<Self, Box<dyn Error>> where Self: Sized;
}