pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for char in string.chars() {
        match char {
            '[' | '{' | '(' => stack.push(char),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
