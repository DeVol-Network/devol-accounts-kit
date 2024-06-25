
pub const CLIENT_MINT_SIZE: usize = 16;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
/// 8 bytes alignment
pub struct ClientMint {
    pub available: u64,
    pub blocked: u64,
} // size: 16 bytes

impl Default for ClientMint {
    fn default() -> Self {
        Self {
            available: 0,
            blocked: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_mint_offsets() {
        assert_eq!(std::mem::size_of::<ClientMint>(), CLIENT_MINT_SIZE);
    }
}
