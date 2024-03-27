pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut candidates: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::<u64>::new();
    loop {
        if let Some(prime) = candidates.clone().into_iter().next() {
            primes.push(prime);
            candidates = candidates
                .into_iter()
                .filter(|number| !(number % prime == 0))
                .collect();
        }
        if candidates.is_empty() {
            break;
        }
    }
    primes
}
