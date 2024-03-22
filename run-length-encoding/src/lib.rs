use std::collections::VecDeque;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    if !source.is_empty() {
        let mut queue: VecDeque<_> = source.chars().collect();
        let mut prev = queue.pop_front().unwrap();
        let mut count = 1;

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if curr == prev {
                count += 1;
            } else {
                if count > 1 {
                    result.push_str(&count.to_string());
                }
                result.push(prev);
                prev = curr;
                count = 1;
            }
        }
        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(prev);
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    if !source.is_empty() {
        let mut queue: VecDeque<_> = source.chars().collect();
        let mut prev = queue.pop_front().unwrap();
        let mut count = 1;

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            if curr == prev {
                count += 1;
            } else {
                if count > 1 {
                    result.push_str(&count.to_string());
                }
                result.push(prev);
                prev = curr;
                count = 1;
            }
        }
        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(prev);
    }
    result
}
