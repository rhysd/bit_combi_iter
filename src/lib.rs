//! This is a small dependency-free crate to enumerate all bit combinations less than given unsigned integer value
//! keeping `1`s in the bits.
//!
//! ```
//! use bit_combi_iter::BitCombinations;
//!
//! let mut c = BitCombinations::new(0b00010100u8);
//!
//! // Iterates all bit combinations less than 0b10100
//! assert_eq!(c.next().unwrap(), 0b00010010);
//! assert_eq!(c.next().unwrap(), 0b00010001);
//! assert_eq!(c.next().unwrap(), 0b00001100);
//! assert_eq!(c.next().unwrap(), 0b00001010);
//! assert_eq!(c.next().unwrap(), 0b00001001);
//! assert_eq!(c.next().unwrap(), 0b00000110);
//! assert_eq!(c.next().unwrap(), 0b00000101);
//! assert_eq!(c.next().unwrap(), 0b00000011);
//!
//! // After iterating all combinations, the iterator reaches the end
//! assert!(c.next().is_none());
//! ```
//!
//! This crate is useful when you want to enumerate all `n` bit integers including `k` ones.
//!
//! ```
//! # use bit_combi_iter::BitCombinations;
//! // Enumerate all 5 bit integers including 3 ones (as u8)
//! let mut c = BitCombinations::new(0b11100u8);
//! assert_eq!(c.next().unwrap(), 0b11010);
//! assert_eq!(c.next().unwrap(), 0b11001);
//! assert_eq!(c.next().unwrap(), 0b10110);
//! assert_eq!(c.next().unwrap(), 0b10101);
//! assert_eq!(c.next().unwrap(), 0b10011);
//! assert_eq!(c.next().unwrap(), 0b01110);
//! // ...
//! ```
//!
//! Implementation of this crate is very efficient:
//!
//! - [The algorithm to generate the next bit combination][1] consists of only a few arithmetic calculations, bit operations and
//!   shift operations.
//! - Size of `BitCombinations<U>` is the same as size of `U`. For example, size of `BitCombinations<u8>` is 8 bits.
//!
//! The algorithm was borrowed from [the blog post][2] by [@herumi][3].
//!
//! [1]: https://github.com/rhysd/bit_combi_iter/blob/ae892b741da82334c9e97afd606c3c5c647fd26b/src/lib.rs#L63-L71
//! [2]: https://github.com/herumi/blog/blob/main/bit-operation.md#%E3%83%93%E3%83%83%E3%83%88%E7%B5%84%E3%81%BF%E5%90%88%E3%82%8F%E3%81%9B%E3%81%AE%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3
//! [3]: https://github.com/herumi

#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro)]

/// A trait to implement the same algorithm for multiple unsigned integer types. [`u8`], [`u16`], [`u32`], [`u64`] and [`u128`] types
/// already implement this trait.
///
/// ```
/// use bit_combi_iter::UIntExt;
///
/// assert_eq!(0b10010u8.next_combination(), Some(0b10001));
/// ```
pub trait UIntExt: Sized + Clone + Copy + PartialEq {
    const ZERO: Self;
    /// Returns the next bit combination of `self`, which is less than `self`. Returns `None` when `self` is the last combination.
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
                    let c = a.wrapping_sub(b >> 1);
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

/// An iterator to iterate all bit combinations less than given unsigned integer value keeping `1`s in the bits. The `U` type
/// parameter can be [`u8`], [`u16`], [`u32`], [`u64`] or [`u128`] primitive types. Size of this struct is exactly the same as
/// size of `U`.
#[derive(Clone, Copy)]
pub struct BitCombinations<U: UIntExt>(U);

impl<U: UIntExt> BitCombinations<U> {
    /// Generates a [`BitCombinations`] instance initializing with state `init`. The instance will iterate all bit combinations
    /// less than `init`.
    ///
    /// ```
    /// # use bit_combi_iter::BitCombinations;
    /// let u = 0b11100u8;
    /// let mut c = BitCombinations::new(u);
    /// assert_eq!(c.peek().unwrap(), u);
    /// ```
    pub fn new(init: U) -> BitCombinations<U> {
        BitCombinations(init)
    }

    /// Returns the current state. If the iterator was already exhausted, it returns `None`.
    ///
    /// ```
    /// # use bit_combi_iter::BitCombinations;
    /// let mut c = BitCombinations::new(0b11100u8);
    /// let u = c.next();
    /// assert_eq!(c.peek(), u);
    /// assert_eq!(c.peek(), c.peek());
    /// ```
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

    /// Returns the next bit combination. When the iterator finished iterating all combinations, it returns `None`.
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
        let u = 0b11000000u8;
        let mut c = BitCombinations::new(u);
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
        let i = c.peek().unwrap();
        assert_eq!(i, u, "left={:b}, right={:b}", i, u);
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
        let u = 0b11111111u8;
        let mut c = BitCombinations::new(u);
        let i = c.peek().unwrap();
        assert_eq!(i, u, "left={:b}, right={:b}", i, u);
        assert!(c.next().is_none());
    }

    #[test]
    fn u8_0() {
        let mut c = BitCombinations::new(0u8);
        assert!(c.peek().is_none());
        assert!(c.next().is_none());
    }
}
