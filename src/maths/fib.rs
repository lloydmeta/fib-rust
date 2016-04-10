/// Returns the fibonacci value at the passed in index
///
/// #Example
/// ```
/// use self::fibonacci::maths::fib::fib_at_index;
/// assert!(fib_at_index(3) == 3);
/// ```
pub fn fib_at_index(i: i64) -> u64 {
    if i < 0 {
        panic!("Got negative index")
    } else if i == 0 || i == 1 {
        1
    } else {
        fib_non_trivial(i)
    }
}

// Called when index is greater than 1
fn fib_non_trivial(i: i64) -> u64 {
    // Rust does not have TOC so we use mutable refs to get around stack overflow problems
    let mut current_index = 2;
    let mut acc_prev = FIB0;
    let mut acc = FIB0 + FIB1;
    while current_index < i {
        let next_prev = acc;
        acc = acc + acc_prev;
        acc_prev = next_prev;
        current_index = current_index + 1;
    }
    acc
}

const FIB0: u64 = 1;
const FIB1: u64 = 1;
