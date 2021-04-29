use std::process;
use std::{env, result::Result};

use sieve_of_eratosthenes::eratosthenes;

fn run_app() -> Result<(), ()> {
    // skip args[0] as that's always the target .exe
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len().ne(&1) {
        eprintln!("error: Please enter exactly one number to test to");
        return Ok(());
    }

    // let target_num: i32 = 50;
    let target_num: i32 = args[0].parse::<i32>().unwrap();

    eratosthenes(target_num);

    return Ok(());
}

fn main() {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {:?}", e);
            1
        }
    });
}
