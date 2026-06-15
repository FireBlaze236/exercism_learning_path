pub fn factors(n: u64) -> Vec<u64> {
    //todo!("This should calculate the prime factors of {n}");
    let mut primes: Vec<u64> = Vec::new();
    let mut number = n;
    let mut candidate = 2;
    while number > 1 {
        while number.is_multiple_of(candidate) {
            primes.push(candidate);
            number /= candidate;
        }
        candidate += 1;
    }

    return primes;
}
