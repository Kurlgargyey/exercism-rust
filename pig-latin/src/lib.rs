use itertools::Itertools;

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|word| translate_word(word))
        .join(" ")
}

fn parse_beginning(input: &str) -> String {
    let vowels = "aeiou";
    let mut beginning = "".to_string();

    for char in input.chars() {
        match char {
            c if c == 'u' && beginning.ends_with("q") => {
                beginning.push(char);
                return beginning;
            }
            _ if beginning == "yt" || beginning == "xr" => {
                return "".to_string();
            }
            c if (c == 'y' && !beginning.is_empty()) || vowels.contains(c) => {
                return beginning;
            }
            _ => {
                beginning.push(char);
            }
        }
    }
    beginning
}

fn translate_word(input: &str) -> String {
    let beginning = parse_beginning(input);
    let mut result = input.chars().skip(beginning.len()).collect::<String>();
    result.push_str(beginning.as_str());
    result.push_str("ay");
    result
}
