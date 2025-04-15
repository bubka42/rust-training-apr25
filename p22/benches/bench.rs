#![feature(test)]
extern crate test;
use p22::calc;
use test::{Bencher, black_box};

#[bench]
fn bench_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| {
        let n = 100;
        black_box(calc::fibonacci_loop(n));
    });
}

#[bench]
fn bench_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| {
        let n = 30;
        black_box(calc::fibonacci_rec(n));
    });
}
