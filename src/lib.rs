// Algorithm borrowed from https://github.com/herumi/misc/blob/d7480b29348841779793edd9d245a834ca1e730f/combination2.cpp

pub trait UIntExt: Sized + Clone + Copy + PartialEq {
    const ZERO: Self;
    fn next_combination(self) -> Option<Self>;
}

macro_rules! impl_uint {
    [$($t:ty)+] => {
        $(
            impl UIntExt for $t {
                const ZERO: Self = 0;
                fn next_combination(self) -> Option<Self> {
                    let a = self;
                    let b = a ^ a.wrapping_add(1);
                    let c = a.wrapping_sub(b / 2);
                    if let Some(d) = b.checked_add(1) {
                        let e = c.wrapping_sub((c & c.wrapping_neg()) >> d.trailing_zeros());
                        if e != 0 { Some(e) } else { None }
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

impl_uint![u8 u16 u32 u64 u128];

#[derive(Clone, Copy)]
pub struct BitCombinations<U: UIntExt>(U);

impl<U: UIntExt> BitCombinations<U> {
    pub fn new(start: U) -> BitCombinations<U> {
        BitCombinations(start)
    }

    pub fn peek(self) -> Option<U> {
        if self.0 == U::ZERO {
            None
        } else {
            Some(self.0)
        }
    }
}

impl<U: UIntExt> Iterator for BitCombinations<U> {
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0.next_combination();
        self.0 = ret.unwrap_or(U::ZERO);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::BitCombinations;
    use std::array;

    #[test]
    fn u8_2() {
        let mut c = BitCombinations::new(0b11000000u8);
        let want = [
            0b10100000u8,
            0b10010000u8,
            0b10001000u8,
            0b10000100u8,
            0b10000010u8,
            0b10000001u8,
            0b01100000u8,
            0b01010000u8,
            0b01001000u8,
            0b01000100u8,
            0b01000010u8,
            0b01000001u8,
            0b00110000u8,
            0b00101000u8,
            0b00100100u8,
            0b00100010u8,
            0b00100001u8,
            0b00011000u8,
            0b00010100u8,
            0b00010010u8,
            0b00010001u8,
            0b00001100u8,
            0b00001010u8,
            0b00001001u8,
            0b00000110u8,
            0b00000101u8,
            0b00000011u8,
        ];
        for j in array::IntoIter::new(want) {
            let i = c.next().unwrap();
            assert_eq!(i, j, "left={:b}, right={:b}", i, j);
            let i = c.peek().unwrap();
            assert_eq!(i, j, "left={:b}, right={:b}", i, j);
        }
        assert!(c.next().is_none());
        assert!(c.peek().is_none());
    }

    #[test]
    fn u8_1() {
        let mut c = BitCombinations::new(0b10000000u8);
        let mut j = 0b10000000u8;
        loop {
            j >>= 1;
            if j == 0 {
                break;
            }
            let i = c.next().unwrap();
            assert_eq!(i, j, "left={:b}, right={:b}", i, j);
        }
        assert!(c.next().is_none());
    }

    #[test]
    fn u8_7() {
        let mut c = BitCombinations::new(0b11111110u8);
        let want = [
            0b11111101u8,
            0b11111011u8,
            0b11110111u8,
            0b11101111u8,
            0b11011111u8,
            0b10111111u8,
            0b01111111u8,
        ];
        for j in array::IntoIter::new(want) {
            let i = c.next().unwrap();
            assert_eq!(i, j, "left={:b}, right={:b}", i, j);
        }
        assert!(c.next().is_none());
    }

    #[test]
    fn u8_8() {
        let mut c = BitCombinations::new(0b11111111u8);
        assert!(c.next().is_none());
    }

    #[test]
    fn u8_0() {
        let mut c = BitCombinations::new(0u8);
        assert!(c.next().is_none());
    }
}
