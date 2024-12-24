use dotenv::dotenv;
use teloxide::prelude::*;

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
}

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting EminorBot...");

    let bot = Bot::from_env();

    let handler = dptree::entry().branch(
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(answer),
    );
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    //TODO: Implement the command handler
    Ok(())
}
