pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    for i in 0.. {
        match n {
            0 => break,
            1 => return Some(i),
            number if number % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}
