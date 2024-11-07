pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes: Vec<u32> = vec![2];
    let mut candidate = 3;

    while primes.len() <= n as usize {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 2; // 짝수는 건너뛰기
    }

    *primes.last().unwrap()
}

fn is_prime(candidate: u32, primes: &[u32]) -> bool {
    let sqrt = (candidate as f64).sqrt() as u32;

    for &prime in primes.iter() {
        if prime > sqrt {
            break;
        }
        if candidate % prime == 0 {
            return false;
        }
    }
    true
}
