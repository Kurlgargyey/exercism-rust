/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        match value.is_palindrome() {
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
//checking for palindromity(?) using 2 inverse char arrays (exits the comparison early on a mismatch and can't overflow)
impl PalindromeCheck for String {
    fn is_palindrome(&self) -> bool {
        self.chars()
            .zip(self.chars().rev())
            .all(|(start, end)| start == end)
    }
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

trait ProductCheck {
    fn is_product_of(&self, min: u64, max: u64) -> bool;
}

impl ProductCheck for u64 {
    fn is_product_of(&self, min: u64, max: u64) -> bool {
        (min..=max).any(|i| { self % i == 0 && (min..=max).contains(&(self / i)) })
    }
}

trait PalindromeProductCheck {
    fn is_palindrome_product_of(&self, min: &u64, max: &u64) -> bool;
    fn new_palindrome_product(&self, min: &u64, max: &u64) -> Option<Palindrome>;
}

impl PalindromeProductCheck for u64 {
    fn is_palindrome_product_of(&self, min: &u64, max: &u64) -> bool {
        self.is_palindrome() && self.is_product_of(*min, *max)
    }

    fn new_palindrome_product(&self, min: &u64, max: &u64) -> Option<Palindrome> {
        if self.is_product_of(*min, *max) { Palindrome::new(*self) } else { None }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut range = min.pow(2)..=max.pow(2);
    let smallest_palindrome = range.find_map(|candidate|
        candidate.new_palindrome_product(&min, &max)
    )?;

    let mut rev_range = range
        .clone()
        .collect::<Vec<u64>>()
        .into_iter()
        .rev()
        .skip_while(|candidate| !candidate.is_palindrome_product_of(&min, &max));

    let largest_palindrome = Palindrome::new(rev_range.next()?)?;
    Some((smallest_palindrome, largest_palindrome))
}
