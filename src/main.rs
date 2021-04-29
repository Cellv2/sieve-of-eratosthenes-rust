fn main() {
    let target_num: i32 = 50;

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
}
