
pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if msg == msg.to_uppercase() && msg != msg.to_lowercase() && msg.ends_with("?") => "Calm down, I know what I'm doing!",
        msg if msg == msg.to_uppercase() && msg != msg.to_lowercase() => "Whoa, chill out!",
        msg if msg.is_empty() => "Fine. Be that way!",
        msg if msg.ends_with("?") => "Sure.",
        _ => "Whatever.",
    }
}
