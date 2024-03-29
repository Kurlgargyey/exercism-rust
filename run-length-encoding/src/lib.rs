use std::collections::VecDeque;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    fn push_char_count(result: &mut String, char: &char, count: &i32) {
        if *count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(*char);
    }
    if !source.is_empty() {
        let mut queue: VecDeque<_> = source.chars().collect();
        let mut prev = queue.pop_front().unwrap();
        let mut count = 1;

        for curr in queue {
            if curr == prev {
                count += 1;
            } else {
                push_char_count(&mut result, &prev, &count);
                prev = curr;
                count = 1;
            }
        }
        push_char_count(&mut result, &prev, &count);
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    if !source.is_empty() {
        let queue: VecDeque<_> = source.chars().collect();
        let mut count_string = String::new();
        let mut count = 1;

        for curr in queue {
            match curr {
                c if c.is_numeric() => count_string.push(curr),
                _ => {
                    if !count_string.is_empty() {
                        count = count_string.parse::<i32>().expect("invalid character count");
                        count_string.clear();
                    }
                    for _ in 0..count {
                        result.push(curr);
                    }
                    count = 1;
                }
            }
        }
    }
    result
}
