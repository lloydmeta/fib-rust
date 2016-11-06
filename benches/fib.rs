#![feature(test)]

extern crate fibonacci;

extern crate test;

use fibonacci::maths::fib::{ fib_at_index, Memoed };
use test::Bencher;

#[bench]
fn bench_fib(b: &mut Bencher) {
    b.iter(|| { fib_at_index(10000) });
}

#[bench]
fn bench_fib_memoed(b: &mut Bencher) {
    let memoed = Memoed::new();
    b.iter(|| { memoed.at_index(10000)})
}