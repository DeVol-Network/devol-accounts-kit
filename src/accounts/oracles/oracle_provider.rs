#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OracleProvider {
    Custom = 0,
    Switchboard = 1,
    Pyth = 2,
}