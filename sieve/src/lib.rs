pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut candidates: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::<u64>::new();
    loop {
        if let Some(candidate) = candidates.clone().into_iter().next() {
            primes.push(candidate);
            candidates = candidates
                .into_iter()
                .filter(|number| !(number % candidate == 0))
                .collect();
        }
        if candidates.is_empty() {
            break;
        }
    }
    primes
}
