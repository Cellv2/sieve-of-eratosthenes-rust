pub fn eratosthenes(limit: i32) {
    let mut primes: Vec<i32> = Vec::new();
    // 2 is the first prime number, so we start there
    let mut nums_to_check: Vec<i32> = (2..limit).collect::<Vec<i32>>();

    for n in 2..limit {
        if nums_to_check.contains(&n) {
            primes.push(n);
            nums_to_check = nums_to_check
                .into_iter()
                .filter(|num| *num % n != 0)
                .collect();
        }
    }

    let largest_prime = primes.clone().into_iter().max().unwrap();
    let all_primes = primes
        .clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("Calculating primes up to {}...", limit);
    println!("Largest prime: {:?}", largest_prime);
    println!("All primes: {}", all_primes);
}