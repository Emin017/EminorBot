use dotenv::dotenv;
use teloxide::prelude::*;

mod parser;
use parser::Command;

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
