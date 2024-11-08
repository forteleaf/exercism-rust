pub fn factors(n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut factors = vec![];
    let mut num = n;
    let mut i = 2;
    while num > 1 {
        while num % i == 0 {
            factors.push(i);
            num /= i;
        }
        i += 1;
    }
    factors
}
