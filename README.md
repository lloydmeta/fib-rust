# Fibonacci in Rust [![Build Status](https://travis-ci.org/lloydmeta/fib-rust.svg?branch=master)](https://travis-ci.org/lloydmeta/fib-rust)

Done as an exercise.

## Areas explored

- Mutex and RWLock (latter replaced former)
- `num` crate
- Having tests in a separate directory

## Running

Command mode:

```bash
$ cargo run -- 10

Fibonacci[10] is 55
```

REPL mode:

```bash
$ cargo run

**************** Interactive Mode ********************
 ctrl+c or ctrl+d or enter any invalid number to exit 
******************************************************

Which Fibonacci index are you interested in?
100
Fibonacci[100] is 354224848179261915075
```

## Tests

`$ cargo test`
