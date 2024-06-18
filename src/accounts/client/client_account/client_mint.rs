
pub const CLIENT_MINT_AVAILABLE_OFFSET: usize = 0;
pub const CLIENT_MINT_BLOCKED_OFFSET: usize = 8;
pub const CLIENT_MINT_SIZE: usize = 16;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct ClientMint {
    pub available: [u8; 8],
    pub blocked: [u8; 8],
} // size: 16 bytes

impl ClientMint {
    #[inline(always)]
    pub fn get_available(&self) -> i64 { i64::from_ne_bytes(self.available) }

    #[inline(always)]
    pub fn set_available(&mut self, value: i64) { self.available = value.to_ne_bytes() }

    #[inline(always)]
    pub fn get_blocked(&self) -> i64 { i64::from_ne_bytes(self.blocked) }

    #[inline(always)]
    pub fn set_blocked(&mut self, value: i64) { self.blocked = value.to_ne_bytes() }
}

impl Default for ClientMint {
    fn default() -> Self {
        Self {
            available: [0; 8],
            blocked: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_mint_offsets() {
        let client_mint = ClientMint::default();

        let base_ptr = &client_mint as *const _ as usize;

        assert_eq!(&client_mint.available as *const _ as usize - base_ptr, CLIENT_MINT_AVAILABLE_OFFSET);
        assert_eq!(&client_mint.blocked as *const _ as usize - base_ptr, CLIENT_MINT_BLOCKED_OFFSET);

        assert_eq!(std::mem::size_of::<ClientMint>(), CLIENT_MINT_SIZE);
    }
}
