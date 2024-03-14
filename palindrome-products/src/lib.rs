/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let string = value.to_string();
        match string.is_palindrome() {
            true => Some(Palindrome(value)),
            false => None,
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
trait PalindromeCheck {
    fn is_palindrome(&self) -> bool;
}

impl PalindromeCheck for String {
    fn is_palindrome(&self) -> bool {
        self.chars()
            .zip(self.chars().rev())
            .all(|(start, end)| start == end)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    todo!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {min} to {max}"
    );
}
