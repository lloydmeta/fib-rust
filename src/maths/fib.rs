use std::sync::Mutex;
use num::bigint::BigUint;
use num::traits::{Zero, One};

/// Returns the fibonacci value at the passed in index
///
/// #Example
/// ```
/// # extern crate num;
/// # extern crate fibonacci;
/// # use num::bigint::ToBigUint;
/// # use self::fibonacci::maths::fib::fib_at_index;
/// # fn main() {
/// assert!(fib_at_index(3) == 2.to_biguint().unwrap());
/// # }
/// ```
pub fn fib_at_index(i: usize) -> BigUint {
    match i {
        0 => Zero::zero(),
        1 => One::one(),
        _ => fib_non_trivial(i),
    }

}

// Called when index is greater than 1
fn fib_non_trivial(i: usize) -> BigUint {
    // Rust does not have TOC so we use mutable refs to get around stack overflow problems
    let mut acc_prev: BigUint = One::one();
    let mut acc: BigUint = One::one();
    for _ in 2 .. i {
        let next_prev = acc.clone();
        acc = acc + acc_prev;
        acc_prev = next_prev;
    }
    acc
}

pub struct Memoed {
    cache: Mutex<Vec<BigUint>>,
}

impl Memoed {
    pub fn new() -> Memoed {
        let mut new_cache = Vec::with_capacity(1000);
        let min_cache = &mut vec![Zero::zero(), One::one()];
        new_cache.append(min_cache);
        Memoed { cache: Mutex::new(new_cache) }
    }

    pub fn at_index(&self, to: usize) -> BigUint {
        let mut data = self.cache.lock().unwrap();

        if data.len() > to {
            data[to].clone()
        } else {
            for current_max in data.len()..to + 1 {
                let sum = data[current_max - 1].clone() + &data[current_max - 2];
                data.push(sum);
            }
            data[to].clone()
        }

    }
}
