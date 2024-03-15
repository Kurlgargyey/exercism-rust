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
    let mut products = generate_products(min, max);
    products.sort();
    println!("sorted vec of products: {:?}", products);
    let min_pal = first_palindrome(&products)?;
    let max_pal = first_palindrome(&products.into_iter().rev().collect())?;
    Some((min_pal, max_pal))
}

fn generate_products(min: u64, max: u64) -> Vec<u64> {
    (0..=max.pow(2))
        .filter(|number| {
            (min..=max).any(|factor1|
                (min..=max).any(|factor2| {
                    let test = number % factor1 == 0 && number / factor1 == factor2;
                    if test {
                        println!("{:?} is the product of {:?} and {:?}!", number, factor1, factor2);
                    }
                    test
                })
            )
        })
        .collect()
}

fn first_palindrome(numbers: &Vec<u64>) -> Option<Palindrome> {
    for i in numbers {
        if let Some(palindrome) = Palindrome::new(*i) {
            return Some(palindrome);
        }
    }
    None
}
