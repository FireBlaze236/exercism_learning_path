pub fn reply(message: &str) -> &str {
    if message.is_empty() || message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    if message.trim().ends_with("?") {
        if message.trim().to_lowercase() == message.trim().to_uppercase() {
            return "Sure.";
        } else if message.trim().to_uppercase() == message {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else if message.trim().to_uppercase() == message.trim().to_lowercase() {
        return "Whatever.";
    } else if message.trim().to_uppercase() == message {
        return "Whoa, chill out!";
    }

    return "Whatever.";
}
