pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut number = n;
    let mut possible_factors = 2..;
    while number > 1 {
        let curr = possible_factors.next().unwrap();
        while number % curr == 0 {
            factors.push(curr);
            number /= curr;
        }
    }

    factors
}
