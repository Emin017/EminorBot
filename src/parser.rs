use teloxide::utils::command::{BotCommands, ParseError};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "setup sending weekly scheduled messages.", parse_with = parse_bool)]
    SetAutoTasks(bool),
    #[command(description = "start auto sending weekly scheduled messages.", parse_with = parse_bool)]
    StartAutoTasks(bool),
}

pub fn parse_bool(input: String) -> Result<(bool,), ParseError> {
    match input.to_lowercase().as_str() {
        "true" | "yes" | "1" => Ok((true,)),
        "false" | "no" | "0" => Ok((false,)),
        _ => Err(ParseError::Custom("Invalid boolean value".into())),
    }
}

#[cfg(test)]
mod parse_bool_tests {
    use crate::parser::parse_bool;
    #[test]
    fn it_works() {
        // Convert string to bool
        let tests = [
            ("true", true),
            ("false", false),
            ("yes", true),
            ("no", false),
            ("1", true),
            ("0", false),
        ];
        for (input, expected) in tests {
            let result = parse_bool(input.to_string());
            let value = result
                .map(|(v,)| v)
                .unwrap_or_else(|_| panic!("Failed to parse '{}'", input));
            assert_eq!(value, expected);
        }
    }
    #[test]
    fn it_fail() {
        let result_eee = parse_bool("eeeeee".to_string());
        assert!(matches!(result_eee, Err(e) if e.to_string() == "Invalid boolean value"));
        let result_ooo = parse_bool("oooooo".to_string());
        assert!(matches!(result_ooo, Err(e) if e.to_string() == "Invalid boolean value"));
    }
}
