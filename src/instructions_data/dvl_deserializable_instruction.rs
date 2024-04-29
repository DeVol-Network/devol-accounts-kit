use crate::dvl_error::DvlError;
use crate::errors::ContractError;

pub trait DvlDeserializableInstruction<'a> {
    fn expected_size() -> usize;
    fn expected_version() -> u8;
    #[inline(always)]
    fn check_version(vec: &'a[u8]) -> Result<(), DvlError> {
        if vec[1] != Self::expected_version() {
            return Err(DvlError::new(ContractError::InstructionDataVersion));
        }
        Ok(())
    }

    fn from_vec_le(vec: &'a[u8]) -> Result<&'a Self, DvlError> where Self: Sized {
        if vec.len() != Self::expected_size() {
            return Err(DvlError::new(ContractError::InstructionDataLength));
        }
        Ok(unsafe { &*(vec.as_ptr() as *const Self) })
    }
}