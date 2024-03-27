const UNITS: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];
const TEENS: &[&str] = &[
    "pad for indexing",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: &[&str] = &[
    "pad for indexing",
    "ten",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];
const LARGER_NUMBERS: &[&str] = &[
    " hundred",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
];
pub fn encode(n: u64) -> String {
    match n {
        0..=999 => encode_small_numbers(n).unwrap(),
        1000..=999_999 => encode_large_numbers(n, 1),
        1_000_000..=999_999_999 => encode_large_numbers(n, 2),
        1_000_000_000..=999_999_999_999 => encode_large_numbers(n, 3),
        1_000_000_000_000..=999_999_999_999_999 => encode_large_numbers(n, 4),
        1_000_000_000_000_000..=999_999_999_999_999_999 => encode_large_numbers(n, 5),
        1_000_000_000_000_000_000..=u64::MAX => encode_large_numbers(n, 6),
    }
}

fn encode_small_numbers(n: u64) -> anyhow::Result<String> {
    match n {
        0..=9 => Ok(crate::UNITS[n as usize].to_string()),
        10..=19 =>
            Ok({
                if n == 10 {
                    crate::TENS[0].to_string()
                } else {
                    crate::TEENS[(n % 10) as usize].to_string()
                }
            }),
        20..=99 =>
            Ok({
                let ten_val = n / 10;
                let unit_val = n % 10;
                let mut result = crate::TENS[ten_val as usize].to_string();
                if unit_val > 0 {
                    result.push('-');
                    result.push_str(crate::UNITS[unit_val as usize]);
                }
                result
            }),
        100..=999 =>
            Ok({
                let hundreds_val = n / 100;
                let remainder_val = n % 100;
                let mut result = crate::UNITS[hundreds_val as usize].to_string();
                result.push_str(crate::LARGER_NUMBERS[0]);
                if remainder_val > 0 {
                    result.push(' ');
                    result.push_str(&encode(remainder_val));
                }
                result
            }),
        _ => Err(anyhow::anyhow!("n out of range!!")),
    }
}

fn encode_large_numbers(n: u64, power_of_thousand: usize) -> String {
    let magnitude_val = n / (1000_u64).pow(power_of_thousand as u32);
    let remainder_val = n % (1000_u64).pow(power_of_thousand as u32);
    let mut result = encode(magnitude_val);
    result.push_str(crate::LARGER_NUMBERS[power_of_thousand]);
    if remainder_val > 0 {
        result.push(' ');
        result.push_str(&encode(remainder_val));
    }
    result
}
