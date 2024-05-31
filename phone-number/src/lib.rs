pub fn number(user_number: &str) -> Option<String> {
    let digits = user_number
        .chars()
        .rev()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<_>>();

    // these are all indexed from the back of the string
    match digits.len() >= 10
        && digits[9].to_digit(10).unwrap() >= 2
        && digits[6].to_digit(10).unwrap() >= 2
        && (digits.len() == 10 || (digits.len() == 11 && digits[10].to_digit(10).unwrap() == 1))
    {
        true => Some(
            digits
                .into_iter()
                .take(10) // take LAST 10 digits
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>(),
        ),
        false => None,
    }
}
