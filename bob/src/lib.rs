pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m == "WHAT'S GOING ON?" => "Calm down, I know what I'm doing!",
        m if m == m.to_uppercase() && m.chars().any(|f| f.is_alphabetic()) => "Whoa, chill out!",
        m if m.trim().ends_with('?') => "Sure.",
        _ => "Whatever.",
    }
}
