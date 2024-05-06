pub fn answer(command: &str) -> Option<i32> {
    if !command.ends_with('?') {
        return None;
    }

    let words: Vec<&str> = command
        .trim_end_matches('?')
        .split_ascii_whitespace()
        .collect();

    match &words[..] {
        &["What", "is", op1 @ _, ref rest @ ..] => execute_command(op1.parse::<i32>().ok()?, rest),
        _ => None,
    }
}

fn execute_command(op1: i32, command: &[&str]) -> Option<i32> {
    match command {
        &["divided", "by", op2 @ _, ref rest @ ..] => {
            execute_command(op1 / op2.parse::<i32>().ok()?, rest)
        }
        &["multiplied", "by", op2 @ _, ref rest @ ..] => {
            execute_command(op1.saturating_mul(op2.parse::<i32>().ok()?), rest)
        }
        &["plus", op2 @ _, ref rest @ ..] => {
            execute_command(op1.saturating_add(op2.parse::<i32>().ok()?), rest)
        }
        &["minus", op2 @ _, ref rest @ ..] => {
            execute_command(op1.saturating_sub(op2.parse::<i32>().ok()?), rest)
        }
        &["raised", "to", "the", op2 @ _, "power", ref rest @ ..] => execute_command(
            op1.saturating_pow(
                op2.trim_end_matches(char::is_alphabetic)
                    .parse::<u32>()
                    .ok()?,
            ),
            rest,
        ),
        &[] => Some(op1),
        _ => None,
    }
}
