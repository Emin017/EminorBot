use dotenv::dotenv;
use teloxide::types::BotCommandScope;
use teloxide::utils::command::BotCommands;
use teloxide::{
    dispatching::{HandlerExt, dialogue::InMemStorage},
    prelude::*,
};

mod parser;
use parser::Command;

mod handle;
use handle::answer;

mod deepseek;

type DialogueState = handle::State;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting EminorBot...");

    let bot = Bot::from_env();

    // Set bot commands
    let commands = Command::bot_commands();
    bot.set_my_commands(commands.clone()).await.unwrap();
    bot.set_my_commands(commands)
        .scope(BotCommandScope::AllGroupChats)
        .await
        .unwrap();

    let storage = InMemStorage::<DialogueState>::new();

    let handler = dptree::entry().branch(
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(
                |bot: Bot, msg: Message, cmd: Command| async move { answer(bot, msg, cmd).await },
            ),
    );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
