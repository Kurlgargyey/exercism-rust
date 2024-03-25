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

    fn encode_large_numbers(
        n: u64,
        number_string: &str,
        order_of_magnitude: u64,
        units: &[&str]
    ) -> String {
        let magnitude_val = n / order_of_magnitude;
        let remainder_val = n % order_of_magnitude;
        let mut result = units[magnitude_val as usize].to_string();
        result.push_str(number_string);
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
        100..=999 => encode_large_numbers(n, " hundred", 100, units),
        1000..=9999 => encode_large_numbers(n, " thousand", 1000, units),
        _ => "".to_string(),
    }
}
