pub fn fib_at_index(i: i64) -> i64 {
  if i < 0 {
    panic!("Got negative index")
  } else if i == 0 || i == 1 {
    1
  } else {
    fib_non_trivial(i)
  }
}

// Called when index is greater than 0
fn fib_non_trivial(i: i64) -> i64 {
  // Rust does not have TOC so we use mutable refs to get around stack overflow problems
  let mut current_index = 2;
  let mut acc_prev = FIB1;
  let mut acc = FIB0 + FIB1;
  while current_index < i {
    let next_prev = acc;
    acc = acc + acc_prev;
    acc_prev = next_prev;
    current_index = current_index + 1;
  }
  acc
}

const FIB0: i64 = 1;
const FIB1: i64 = 1;
