use std::error::Error;
use crate::account_readers::dvl_account_reader::DvlAccountReader;

pub trait DvlReadable {
    fn read(reader: &DvlAccountReader) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized;
}
