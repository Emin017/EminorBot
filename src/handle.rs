use std::env;
use std::error::Error;
use teloxide::utils::command::BotCommands;
use teloxide::{
    prelude::*,
    types::{Message as TelegramMessage, ParseMode},
};

use crate::deepseek::DeepSeekClient;
use crate::parser::Command;

pub type Message = TelegramMessage;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    log::info!("Command received: {:?}", cmd);

    match cmd {
        Command::Help => {
            log::info!("Sending help message...");
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Ask(question) => {
            // Extract question and mention
            log::info!("Extracting question and mention...");

            let is_command_mention = msg
                .text()
                .map(|text| text.starts_with("/ask@") || text.starts_with("/ask @"))
                .unwrap_or(false);

            let (question_text, is_text_mentioned) = extract_question_and_mention(&question);
            let is_mentioned = is_command_mention || is_text_mentioned;

            // If not mentioned in group, then ignore it.
            if !is_mentioned && (msg.chat.is_group() || msg.chat.is_supergroup()) {
                log::info!("Ignoring message not mentioning the bot in group chat...");
                return Ok(());
            }

            match check_is_question_empty(&question) {
                true => {
                    return Ok(());
                }
                false => {}
            }

            log::info!("Asking DeepSeek: {}", question_text);
            log::info!("Thinking...");
            bot.send_message(msg.chat.id, "Thinking...").await?;

            match ask_deep_seek(&question_text).await {
                Ok(response) => {
                    // Send response
                    send_chunked_response(&bot, msg.chat.id, &response).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Sorry, Error: {}", e))
                        .await?;
                }
            }
        }
    }
    Ok(())
}

fn check_is_question_empty(question: &str) -> bool {
    match question.trim().is_empty() {
        true => {
            log::info!("No question provided...");
            true
        }
        false => false,
    }
}

/// Extract question and check if it mentions @eminor_e_bot
fn extract_question_and_mention(input: &str) -> (String, bool) {
    let bot_name = env::var("BOT_USERNAME").unwrap_or_else(|_| "eminor_e_bot".to_string());
    let mention = format!("@{}", bot_name);
    log::info!("Bot mention: {}", mention);

    // Check if it mentions the bot
    let is_mentioned = input.contains(&mention);

    log::info!("Is mentioned: {}", is_mentioned);
    let question = match is_mentioned {
        true => input.replace(&mention, "").trim().to_string(),
        false => input.trim().to_string(),
    };

    (question, is_mentioned)
}

// Send chunked response in long message
async fn send_chunked_response(bot: &Bot, chat_id: ChatId, response: &str) -> ResponseResult<()> {
    let mut remaining = response;
    log::info!("Sending chunked response...");
    while !remaining.is_empty() {
        let chunk_size = remaining.len().min(4000); // Telegram message limit
        let (chunk, rest) = remaining.split_at(chunk_size);

        // Try to send message
        match bot
            .send_message(chat_id, chunk)
            .parse_mode(ParseMode::MarkdownV2)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                // If fail to parse markdown, then
                log::warn!("Failed to send message with MarkdownV2: {}", e);
                bot.send_message(chat_id, chunk).await?;
            }
        }

        remaining = rest;
    }

    Ok(())
}

pub async fn ask_deep_seek(question: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let deep_seek = DeepSeekClient::new()?;
    deep_seek.send_message(question).await
}
