use std::process;
use std::{env, result::Result};

fn run_app() -> Result<(), ()> {
    // skip args[0] as that's always the target .exe
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len().ne(&1) {
        eprintln!("error: Please enter exactly one number to test to");
        return Ok(());
    }

    // let target_num: i32 = 50;
    let target_num: i32 = args[0].parse::<i32>().unwrap();

    let mut primes: Vec<i32> = Vec::new();
    // 2 is the first prime number, so we start there
    let mut nums_to_check: Vec<i32> = (2..target_num).collect::<Vec<i32>>();

    for n in 2..target_num {
        if nums_to_check.contains(&n) {
            primes.push(n);
            nums_to_check = nums_to_check
                .into_iter()
                .filter(|num| *num % n != 0)
                .collect();
        }
    }

    let largest_prime = primes.clone().into_iter().max();
    let all_primes = primes
        .clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("Calculating primes up to {}...", target_num);
    println!("Largest prime: {:?}", largest_prime.unwrap());
    println!("All primes: {}", all_primes);

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
