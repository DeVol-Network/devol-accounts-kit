#[derive(Clone, PartialEq)]
#[repr(i64)]
pub enum KYCStatus {
    Blocked = -1,
    Light = 0,
    Power = 1000000,
    PowerLp = 1000001,
}