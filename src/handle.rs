use teloxide::{prelude::*, types::Message};

use crate::parser::Command;

pub async fn answer(_bot: Bot, _msg: Message, _cmd: Command) -> ResponseResult<()> {
    //TODO: Implement the command handler
    Ok(())
}
