use teloxide::prelude::Message;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(
        description = "ask DeepSeek a question directly.",
        parse_with = "split"
    )]
    Ask(String),
}

/// This function parses custom commands that are not handled by the command dispatcher.
/// e.g. /ask @eminor_e_bot What is the meaning of life?
/// The default command dispatcher does not handle this, it only handles commands like /ask@eminor_e_bot,
/// So we need to parse this custom command manually.
pub fn parse_custom_commands(msg: Message) -> Option<(Message, Command)> {
    if let Some(text) = msg.text() {
        // Check if it starts with "/ask "
        if text.starts_with("/ask ") {
            // Parse the question
            let rest = &text[5..]; // Skip "/ask "

            // Check if the bot is mentioned
            let bot_name =
                std::env::var("BOT_USERNAME").unwrap_or_else(|_| "eminor_e_bot".to_string());
            let mention = format!("@{}", bot_name);

            let question = match rest.starts_with(&mention) {
                true => {
                    // Remove mention and trim the rest
                    rest[mention.len()..].trim().to_string()
                }
                false => {
                    // If not mentioned, just trim the rest
                    rest.trim().to_string()
                }
            };

            log::info!("Parsed custom command: Ask with question: {}", question);
            return Some((msg, Command::Ask(question)));
        }
    }
    None
}
