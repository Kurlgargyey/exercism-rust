pub fn encode(n: u64) -> String {
    let units: &[&str; 10] = &[
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

    fn encode_large_numbers(
        n: u64,
        number_strings: &[&str],
        order_of_magnitude: u32,
        units: &[&str]
    ) -> String {
        let magnitude_val = n / (10u64).pow(order_of_magnitude);
        let remainder_val = n % (10u64).pow(order_of_magnitude);
        let mut result = units[magnitude_val as usize].to_string();
        result.push_str(number_strings[(order_of_magnitude - 3) as usize]);
        if remainder_val > 0 {
            result.push(' ');
            result.push_str(encode(remainder_val).as_str());
        }
        result
    }

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

        _ => encode_large_numbers(n, larger_numbers, n.checked_ilog10().unwrap_or(0) + 1, units),
    }
}
