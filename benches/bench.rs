#![feature(test)]

extern crate test;

use bit_combi_iter::BitCombinations;
use test::Bencher;

macro_rules! benchmark {
    ($name:ident, $init:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let u = $init;
                for i in BitCombinations::new(u) {
                    test::black_box(i);
                }
            })
        }
    };
}

benchmark!(u8_3, 0b111u8 << 5);
benchmark!(u8_5, 0b11111u8 << 3);
benchmark!(u8_1, 0b1u8 << 7);
benchmark!(u32_3, 0b111u32 << 29);
benchmark!(u64_3, 0b111u64 << 61);
