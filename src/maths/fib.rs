use std::sync::{Arc, Mutex};
//use num_bigint::BigUint;
//use num_traits::{Zero, One};

/// Returns the fibonacci value at the passed in index
///
/// #Example
/// ```
/// use self::fibonacci::maths::fib::fib_at_index;
/// assert!(fib_at_index(3) == 2);
/// ```
pub fn fib_at_index(i: i64) -> usize {
    if i <= 1 {
        i as usize
    } else {
        fib_non_trivial(i)
    }
}

// Called when index is greater than 1
fn fib_non_trivial(i: i64) -> usize {
    // Rust does not have TOC so we use mutable refs to get around stack overflow problems
    let mut current_index = 2;
    let mut acc_prev = 1;
    let mut acc = 1;
    while current_index < i {
        let next_prev = acc;
        acc = acc + acc_prev;
        acc_prev = next_prev;
        current_index += 1;
    }
    acc
}

pub struct Memoed {
    cache: Arc<Mutex<Vec<usize>>>,
}

impl Memoed {
    pub fn new() -> Memoed {
        let mut new_cache = Vec::with_capacity(1000);
        let min_cache = &mut vec![0, 1];
        new_cache.append(min_cache);
        Memoed { cache: Arc::new(Mutex::new(new_cache)) }
    }

    pub fn at_index(&self, to: usize) -> usize {
        let cloned = self.cache.clone();
        let mut data = cloned.lock().unwrap();

        if data.len() > to {
            data[to]
        } else {
            for current_max in data.len()..to + 1 {
                let sum = data[current_max - 1] + data[current_max - 2];
                data.push(sum);
            }
            data[to]
        }

    }
}
