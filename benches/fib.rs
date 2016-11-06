#![feature(test)]

extern crate fibonacci;

extern crate test;

use fibonacci::maths::fib::{ fib_at_index, Memoed };
use test::Bencher;

#[bench]
fn bench_fib30(b: &mut Bencher) {
    b.iter(|| { fib_at_index(30) });
}

#[bench]
fn bench_fib30_memoed(b: &mut Bencher) {
    let memoed = Memoed::new();
    b.iter(|| { memoed.at_index(30)})
}