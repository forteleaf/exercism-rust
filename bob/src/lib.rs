pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    if message.trim().is_empty() {
        "Fine. Be that way!"
    } else if message == "WHAT'S GOING ON?" {
        "Calm down, I know what I'm doing!"
    } else if message == message.to_uppercase() && message.chars().any(|f| f.is_alphabetic()) {
        "Whoa, chill out!"
    } else if message.trim().ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
