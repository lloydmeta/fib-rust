#![feature(test)]

extern crate fibonacci;

extern crate test;

use fibonacci::maths::fib::fib_at_index;
use test::Bencher;

#[bench]
fn bench_fib30(b: &mut Bencher) {
    b.iter(|| { fib_at_index(30)});
}