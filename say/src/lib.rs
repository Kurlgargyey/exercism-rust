use std::arch::x86_64::_mm_stream_si64;

pub fn encode(n: u64) -> String {
    let units = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = &[
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
    let tens = &[
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

    let larger_numbers = &[
        " hundred",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
    ];

    match n {
        0..=9 => units[n as usize].to_string(),
        10..=19 => {
            if n == 10 { tens[0].to_string() } else { teens[(n % 10) as usize].to_string() }
        }
        20..=99 => {
            let ten_val = n / 10;
            let unit_val = n % 10;
            let mut result = tens[ten_val as usize].to_string();
            if unit_val > 0 {
                result.push('-');
                result.push_str(units[unit_val as usize]);
            }
            result
        }
        100..=999 => {
            let hundreds_val = n / 100;
            let remainder_val = n % 100;
            let mut result = units[hundreds_val as usize].to_string();
            result.push_str(larger_numbers[0]);
            if remainder_val > 0 {
                result.push(' ');
                result.push_str(&encode(remainder_val));
            }
            result
        }
        1000..=999_999 => { encode_large_numbers(n, larger_numbers, 1) }
        1_000_000..=999_999_999 => { encode_large_numbers(n, larger_numbers, 2) }
        1_000_000_000..=999_999_999_999 => { encode_large_numbers(n, larger_numbers, 3) }
        1_000_000_000_000..=999_999_999_999_999 => { encode_large_numbers(n, larger_numbers, 4) }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            encode_large_numbers(n, larger_numbers, 5)
        }
        1_000_000_000_000_000_000..=u64::MAX => { encode_large_numbers(n, larger_numbers, 6) }

        _ => "".to_string(),
    }
}

fn encode_large_numbers(n: u64, number_strings: &[&str], power_of_thousand: usize) -> String {
    let magnitude_val = n / (1000_u64).pow(power_of_thousand as u32);
    let remainder_val = n % (1000_u64).pow(power_of_thousand as u32);
    let mut result = encode(magnitude_val);
    result.push_str(number_strings[power_of_thousand]);
    if remainder_val > 0 {
        result.push(' ');
        result.push_str(&encode(remainder_val));
    }
    result
}
