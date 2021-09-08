#![feature(test)]

extern crate test;

use bit_combi_iter::BitCombinations;
use test::Bencher;

#[bench]
fn u8_3(b: &mut Bencher) {
    b.iter(|| {
        let u = 0b111u8 << 5;
        for i in BitCombinations::new(u) {
            test::black_box(i);
        }
    })
}

#[bench]
fn u8_5(b: &mut Bencher) {
    b.iter(|| {
        let u = 0b11111u8 << 3;
        for i in BitCombinations::new(u) {
            test::black_box(i);
        }
    })
}

#[bench]
fn u8_1(b: &mut Bencher) {
    b.iter(|| {
        let u = 0b1u8 << 7;
        for i in BitCombinations::new(u) {
            test::black_box(i);
        }
    })
}

#[bench]
fn u32_3(b: &mut Bencher) {
    b.iter(|| {
        let u = 0b111u32 << 29;
        for i in BitCombinations::new(u) {
            test::black_box(i);
        }
    })
}

#[bench]
fn u64_3(b: &mut Bencher) {
    b.iter(|| {
        let u = 0b111u64 << 61;
        for i in BitCombinations::new(u) {
            test::black_box(i);
        }
    })
}
