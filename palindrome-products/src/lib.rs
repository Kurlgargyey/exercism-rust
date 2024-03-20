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
}

impl PalindromeProductCheck for u64 {
    fn is_palindrome_product_of(&self, min: &u64, max: &u64) -> bool {
        self.to_string().is_palindrome() && self.is_product_of(*min, *max)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    /*
    let mut products = generate_products(min, max);
    products.sort();
    println!("sorted vec of products: {:?}", products);
    */

    let mut range = (min.pow(2)..=max.pow(2)).skip_while(
        |candidate| !candidate.is_palindrome_product_of(&min, &max)
    );
    let mut rev_range = range
        .clone()
        .collect::<Vec<u64>>()
        .into_iter()
        .rev()
        .skip_while(|candidate| !candidate.is_palindrome_product_of(&min, &max));

    let min_pal = Palindrome::new(range.next()?)?;
    let max_pal = Palindrome::new(rev_range.next()?)?;
    Some((min_pal, max_pal))
}

fn generate_products(min: u64, max: u64) -> Vec<u64> {
    let mut products = Vec::<u64>::new();
    let mut range: Vec<u64> = (min.pow(2)..=max.pow(2)).collect();

    for i in min..=max {
        range.retain(|number| {
            if number % i == 0 && (min..=max).contains(&(number / i)) {
                products.push(*number);
                return false;
            }
            true
        });
    }

    products.into_iter().collect()
    /*
    (min..=max)
        .fold(products, |mut set, number| {
            for number2 in min..=max {
                set.insert(number * number2);
            }
            set
        })
        .into_iter()
        .collect()
    */
    /*
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
    */
}

fn first_palindrome(numbers: &Vec<u64>) -> Option<Palindrome> {
    for i in numbers {
        if let Some(palindrome) = Palindrome::new(*i) {
            return Some(palindrome);
        }
    }
    None
}
