extern crate fibonacci;
use self::fibonacci::maths::fib::fib_at_index;

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
        i = i + 1;
    }
}
