pub fn encode(n: u64) -> String {
    let strings = &[
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
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "teen",
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
        "hundred",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
    ];

    match n {
        0..=13 => strings[n as usize].to_string(),
        14..=19 => {
            let mut string = encode(
                n.to_string().chars().nth(1).unwrap().to_string().parse::<u64>().unwrap()
            );
            string.push_str("teen");
            string
        }
        20 => "twenty".to_string(),
        21..=29 => {
            let mut string = "twenty-".to_string();

            string.push_str(
                &encode(n.to_string().chars().nth(1).unwrap().to_string().parse::<u64>().unwrap())
            );
            string
        }
        30 => "thirty".to_string(),
        31..=39 => {
            let mut string = "thirty-".to_string();

            string.push_str(
                &encode(n.to_string().chars().nth(1).unwrap().to_string().parse::<u64>().unwrap())
            );
            string
        }

        _ => "".to_string(),
    }
}
