#![feature(test)]
extern crate test;

use p54::*;
use test::{black_box, Bencher};

#[bench]
fn bench_encrypt8(b: &mut Bencher) {
    let key = black_box([11u8; 16]);
    let data = black_box([0u8; 128]);
    let round_keys = unsafe { expand_key(&key) };
    b.iter(|| {
        black_box(unsafe { encrypt8(&round_keys, &data) });
    });
    b.bytes = 128;
}

#[bench]
fn bench_decrypt8(b: &mut Bencher) {
    let key = black_box([7u8; 16]);
    let data = black_box([0u8; 128]);
    let round_keys = unsafe { expand_key(&key) };
    b.iter(|| {
        black_box(unsafe { decrypt8(&round_keys, &data) });
    });
    b.bytes = 128;
}
