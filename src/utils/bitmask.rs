use std::ops::{BitAnd, BitOr, BitXor, Not, Shl};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Serialize, Deserialize)]
pub struct BitMask<T>(pub T);

impl<T> BitMask<T>
    where
        T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Not<Output = T> + Shl<u8, Output = T> + From<u8> + Copy + PartialEq,
{
    #[inline(always)]
    pub fn set(&mut self, index: u8, value: bool) {
        if value {
            self.0 = self.0 | (T::from(1) << index);
        } else {
            self.0 = self.0 & !(T::from(1) << index);
        }
    }

    #[inline(always)]
    pub fn get(&self, index: u8) -> bool {
        (self.0 & (T::from(1) << index)) != T::from(0)
    }

    #[inline(always)]
    fn value(&self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_u32() {
        let mut mask = BitMask(0u32);
        mask.set(0, true);
        assert!(mask.get(0));
        assert_eq!(mask.value(), 1);

        mask.set(31, true);
        assert!(mask.get(31));
        assert_eq!(mask.value(), 0x80000001);

        mask.set(0, false);
        assert!(!mask.get(0));
        assert_eq!(mask.value(), 0x80000000);
    }

    #[test]
    fn test_set_get_u64() {
        let mut mask = BitMask(0u64);
        mask.set(0, true);
        assert!(mask.get(0));
        assert_eq!(mask.value(), 1);

        mask.set(63, true);
        assert!(mask.get(63));
        assert_eq!(mask.value(), 0x8000000000000001);

        mask.set(0, false);
        assert!(!mask.get(0));
        assert_eq!(mask.value(), 0x8000000000000000);
    }

    #[test]
    fn test_multiple_flags_u32() {
        let mut mask = BitMask(0u32);
        mask.set(0, true);
        mask.set(15, true);
        mask.set(31, true);
        assert!(mask.get(0));
        assert!(mask.get(15));
        assert!(mask.get(31));
        assert_eq!(mask.value(), 0x80008001);
    }

    #[test]
    fn test_multiple_flags_u64() {
        let mut mask = BitMask(0u64);
        mask.set(0, true);
        mask.set(31, true);
        mask.set(63, true);
        assert!(mask.get(0));
        assert!(mask.get(31));
        assert!(mask.get(63));
        assert_eq!(mask.value(), 0x8000000080000001);
    }
}