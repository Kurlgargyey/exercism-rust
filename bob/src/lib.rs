pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    let message_is_nothing = message.is_empty();
    let message_is_shouting =
        message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic());
    let message_is_question = message.ends_with('?');
    match (message_is_question, message_is_shouting, message_is_nothing) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Sure.",
        (false, false, true) => "Fine. Be that way!",
        (false, true, false) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
