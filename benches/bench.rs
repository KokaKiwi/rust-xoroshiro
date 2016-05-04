#![feature(test)]
extern crate rand;
extern crate test;
extern crate xoroshiro;

const BENCH_N: u64 = 1000;

use std::mem::size_of;
use rand::{OsRng, Rng};
use test::{black_box, Bencher};
use xoroshiro::Xoroshiro128Plus;

#[bench]
fn bench_xorshiro128plus(b: &mut Bencher) {
    let mut rng: Xoroshiro128Plus = OsRng::new().unwrap().gen();
    b.iter(|| {
        for _ in 0..BENCH_N {
            black_box(rng.gen::<usize>());
        }
    });
    b.bytes = BENCH_N * size_of::<usize>() as u64;
}
