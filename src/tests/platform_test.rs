#[cfg(test)]
mod tests {
    use std::mem::transmute;

    #[test]
    fn machine_is_little_endian() {
        let num: u16 = 0x0F;
        let bytes: [u8; 2] = unsafe { transmute(num) };
        assert_eq!(bytes[0], 0xF, "This project is intended for use only in a little-endian environment and is not compatible with big-endian architectures.");
    }
}