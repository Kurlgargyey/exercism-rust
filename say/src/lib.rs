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
            result.push_str(" hundred");
            if remainder_val > 0 {
                result.push(' ');
                result.push_str(encode(remainder_val).as_str());
            }
            result
        }
        1000..=9999 => {
            let thousands_val = n / 1000;
            let remainder_val = n % 1000;
            let mut result = units[thousands_val as usize].to_string();
            result.push_str(" thousand");
            if remainder_val > 0 {
                result.push(' ');
                result.push_str(encode(remainder_val).as_str());
            }
            result
        }
        _ => "".to_string(),
    }
}
