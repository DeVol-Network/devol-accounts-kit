use crate::dvl_error::DvlError;
use crate::errors::ContractError;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i64)]
pub enum KYCStatus {
    Blocked = -1,
    Light = 0,
    Power = 1000000,
    PowerLp = 1000001,
}

impl TryFrom<i64> for KYCStatus {
    type Error = DvlError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(KYCStatus::Blocked),
            0 => Ok(KYCStatus::Light),
            1_000_000 => Ok(KYCStatus::Power),
            1_000_001 => Ok(KYCStatus::PowerLp),
            _ =>  Err(DvlError::new(ContractError::InvalidKycStatus)),
        }
    }
}