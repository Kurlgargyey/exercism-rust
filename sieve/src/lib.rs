pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut candidate_vec: Vec<u64> = (2..=upper_bound).collect();
    let mut result: Vec<u64> = Vec::<u64>::new();
    loop {
        if let Some(candidate) = candidate_vec.clone().into_iter().next() {
            result.push(candidate);
            candidate_vec = candidate_vec
                .into_iter()
                .filter(|number| !(number % candidate == 0))
                .collect();
        }
        if candidate_vec.is_empty() {
            break;
        }
    }
    result
}
