extern crate fibonacci;

use self::fibonacci::maths::fib::{ fib_at_index, Memoed };
use std::thread;
use std::sync::Arc;
use num::bigint::{ ToBigUint, BigUint };
use num::traits::{Zero, One};

#[test]
fn test_fib0() {
    assert_eq!(fib_at_index(0), Zero::zero());
}

#[test]
fn test_fib1() {
    assert_eq!(fib_at_index(1), One::one());
}

#[test]
fn test_fib_after_1() {
    let mut i = 3;
    let mut prev: BigUint = One::one();
    let mut current: BigUint = One::one();
    while i < 90 {
        let next_prev = current.clone();
        let expected = prev + &next_prev;
        current = fib_at_index(i);
        assert_eq!(current, expected);
        prev = next_prev.clone();
        i += 1;
    }
}

#[test]
fn test_memoed() {
    let memoed = Memoed::new();
    assert_eq!(*memoed.at_index(10), 55.to_biguint().unwrap())
}

#[test]
fn test_memoed_multithreaded() {
    let memoed = Arc::new(Memoed::new());

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for _ in 0..100 {
        let memoed = memoed.clone();
        let t = thread::spawn(move || {
            for i in 0 .. 30 {
                let f = memoed.at_index(i);
                assert_eq!(*f, fib_at_index(i as usize));
            }
        });
        children.push(t);
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}