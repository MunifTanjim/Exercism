pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    if message.len() == 0 {
        return "Fine. Be that way!";
    }

    let is_all_caps = message.chars().any(char::is_alphabetic) && message.to_uppercase() == message;
    let is_question = message.ends_with('?');

    match (is_all_caps, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        (_, _) => "Whatever.",
    }
}
