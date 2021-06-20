enum MessageType {
    Yelled,
    YelledQuestion,
    Question,
    Empty,
    Unknown,
}

impl From<&str> for MessageType {
    fn from(message: &str) -> Self {
        let trimmed = message.trim().chars();
        let is_question = trimmed.clone().last().unwrap_or_default() == '?';
        let is_yelled = {
            let mut ascii_content = trimmed.clone().filter(|c| c.is_ascii_alphabetic());

            ascii_content.clone().count() > 0 && ascii_content.all(|c| c.is_uppercase())
        };

        if is_yelled && is_question {
            return MessageType::YelledQuestion;
        } else if is_yelled {
            return MessageType::Yelled;
        } else if is_question {
            return MessageType::Question;
        } else if message.trim().is_empty() {
            return MessageType::Empty;
        }

        MessageType::Unknown
    }
}

pub fn reply(message: &str) -> &str {
    match MessageType::from(message) {
        MessageType::YelledQuestion => "Calm down, I know what I'm doing!",
        MessageType::Yelled => "Whoa, chill out!",
        MessageType::Question => "Sure.",
        MessageType::Empty => "Fine. Be that way!",
        MessageType::Unknown => "Whatever.",
    }
}
