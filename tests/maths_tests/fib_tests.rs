extern crate fibonacci;

use self::fibonacci::maths::fib::{ fib_at_index, Memoed };
use std::thread;
use std::sync::Arc;

#[test]
fn test_fib0() {
    assert_eq!(fib_at_index(0), 0);
}

#[test]
fn test_fib1() {
    assert_eq!(fib_at_index(1), 1);
}

#[test]
fn test_fib_after_1() {
    let mut i = 3;
    let mut prev = 1;
    let mut current = 1;
    while i < 90 {
        let next_prev = current;
        current = fib_at_index(i);
        assert_eq!(current, (prev + next_prev));
        prev = next_prev;
        i += 1;
    }
}

#[test]
fn test_memoed() {
    let memoed = Memoed::new();
    assert_eq!(memoed.at_index(10), 55)
}

#[test]
fn test_memoed_multithreaded() {
    let memoed = Arc::new(Memoed::new());
    for _ in 0..10 {
        let memoed = memoed.clone();
        let _ = thread::spawn(move || {
            for i in 0 .. 30 {
                let f = memoed.at_index(i);
                assert_eq!(f, fib_at_index(i as i64));
            }
        });
    }
}