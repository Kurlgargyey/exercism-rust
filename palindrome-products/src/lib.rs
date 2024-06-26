#![feature(test)]
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        value.is_palindrome().then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
trait PalindromeCheckString {
    fn is_palindrome_string(&self) -> bool;
}
//checking for palindromity(?) using 2 inverse char arrays (exits the comparison early on a mismatch and can't overflow)
impl PalindromeCheckString for u64 {
    fn is_palindrome_string(&self) -> bool {
        let string = self.to_string();
        string
            .chars()
            .zip(string.chars().rev())
            .all(|(start, end)| start == end)
    }
}

trait PalindromeCheck {
    fn is_palindrome(&self) -> bool;
}
//checking for palindromity(?) using integer math and building a reversed number to compare
impl PalindromeCheck for u64 {
    fn is_palindrome(&self) -> bool {
        let mut number = *self;
        let mut reversed_number = 0;
        while number > 0 {
            let last_digit = number % 10;
            reversed_number = reversed_number * 10 + last_digit;
            number /= 10;
        }
        self == &reversed_number
    }
}

trait PalindromeLogCheck {
    fn is_palindrome_log(&self) -> bool;
}
//checking for palindromity(?) using integer math without the risk for overflowing
impl PalindromeLogCheck for u64 {
    fn is_palindrome_log(&self) -> bool {
        std::iter::successors(
            Some((
                (10u64).pow(self.checked_ilog10().or(Some(1)).unwrap()),
                1u64,
            )),
            |&(hi, lo)| (hi > lo * 100).then_some((hi / 10, lo * 10)),
        )
        .all(|(hi, lo)| (self / hi) % 10 == (self / lo) % 10)
    }
}

trait ProductCheck {
    fn is_product_of(&self, min: &u64, max: &u64) -> bool;
}

impl ProductCheck for u64 {
    fn is_product_of(&self, min: &u64, max: &u64) -> bool {
        (*min..=*max).any(|i| self % i == 0 && (min..=max).contains(&&(self / i)))
    }
}

trait PalindromeProductCheck {
    fn is_palindrome_product_of(&self, min: &u64, max: &u64) -> bool;
}

impl PalindromeProductCheck for u64 {
    fn is_palindrome_product_of(&self, min: &u64, max: &u64) -> bool {
        self.is_palindrome_string() && self.is_product_of(min, max)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let valid_range = min.pow(2)..=max.pow(2);
    let smallest_palindrome = Palindrome::new(
        valid_range
            .clone()
            .find(|candidate| candidate.is_palindrome_product_of(&min, &max))?,
    )?;

    let largest_palindrome = Palindrome::new(
        valid_range
            .rev()
            .find(|candidate| candidate.is_palindrome_product_of(&min, &max))?,
    )?;

    Some((smallest_palindrome, largest_palindrome))
}

fn palindrome_products_iter(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    (min..=max)
        .flat_map(|i| (i..=max).filter_map(move |j| Palindrome::new(i * j)))
        .fold(None, |acc, p| {
            acc.map(|(min, max)| (min.min(p), max.max(p)))
                .or(Some((p, p)))
        })
}

extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use test::Bencher;
    #[bench]
    fn palindrome_products_bench(b: &mut Bencher) {
        let min = 10;
        let max = 99;
        b.iter(|| palindrome_products(min, max));
    }
    #[bench]
    fn palindrome_products_iter_bench(b: &mut Bencher) {
        let min = 10;
        let max = 99;
        b.iter(|| palindrome_products(min, max));
    }

    const MIN: u64 = (2_u64).pow(20);
    const MAX: u64 = (2_u64).pow(21);
    #[bench]
    fn is_palindrome_log_bench(b: &mut Bencher) {
        b.iter(|| {
            for number in MIN..=MAX {
                number.is_palindrome_log();
            }
        });
    }
    #[bench]
    fn is_palindrome_string_bench(b: &mut Bencher) {
        b.iter(|| {
            for number in MIN..=MAX {
                number.is_palindrome_string();
            }
        });
    }
    #[bench]
    fn is_palindrome_bench(b: &mut Bencher) {
        b.iter(|| {
            for number in MIN..=MAX {
                number.is_palindrome();
            }
        });
    }
}
