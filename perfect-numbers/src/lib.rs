#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match aliquot(num) {
        i if i == num => Some(Classification::Perfect),
        i if i > num => Some(Classification::Abundant),
        i if i < num => Some(Classification::Deficient),
        _ => None,
    }
}

fn aliquot(num: u64) -> u64 {
    (1..num).fold(0, |acc, number| if num % number == 0 { acc + number } else { acc })
}
