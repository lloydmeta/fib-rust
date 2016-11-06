extern crate num;
extern crate fibonacci;

use std::io;
use std::num::ParseIntError;
use std::env::args;
use std::io::stdin;

use num::bigint::BigUint;
use fibonacci::maths::fib::{ Memoed, fib_at_index };

fn main() {
    let args: Vec<String> = args().collect();
    // First argument is usually the path of the executable
    if let Some(s) = args.get(1) {
        one_off_mod(s.trim().to_owned())
    } else {
        interactive_mode()
    }.unwrap()
}

fn one_off_mod(input: String) ->  Result<(), CliError> {
    let index = try!(parse_positive_num(input));
    let fib_value = fib_at_index(index);
    Ok(print_result(index, fib_value))
}

fn interactive_mode() -> Result<(), CliError> {
    println!("\n**************** Interactive Mode ********************");
    println!(" ctrl+c or ctrl+d or enter any invalid number to exit ");
    println!("******************************************************");
    let fib_gen = Memoed::new();
    loop {
        println!("Which Fibonacci index are you interested in?");
        let input = try!(read_stdio());
        let index = try!(parse_positive_num(input));
        let fib_value = fib_gen.at_index(index);
        print_result(index, fib_value)
    }
}

fn print_result(idx: usize, fib: BigUint) {
    println!("Fibonacci[{}] is {}", idx, fib)
}

fn parse_positive_num(s: String) -> Result<usize, CliError> {
    match s.trim().parse::<usize>() {
        Ok(n) => Ok(n),
        Err(err) => Err(CliError::Parse(err)),
    }
}

fn read_stdio() -> Result<String, CliError> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .map(|_| input)
        .map_err(CliError::Io)
}

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(ParseIntError),
}
