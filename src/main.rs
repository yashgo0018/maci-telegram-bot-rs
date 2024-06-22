mod models;
mod schema;
mod database;

use self::models::*;
use dotenvy::dotenv;

use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{MessageKind, MessageLeftChatMember, MessageNewChatMembers};
use crate::database::{create_user, establish_connection};

#[tokio::main]
async fn main() {

    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let connection = &mut establish_connection();

        // if the message is a command "/vote option mention" then continue else return
        // print the chat id
        let chat_id = msg.chat.id.0;
        println!("Chat ID: {}", chat_id);

        if let Some(sender) = msg.from() {
            create_user(connection, sender);
            println!("Sender ID: {:?}", sender);
        }

        match &msg.kind {
            MessageKind::NewChatMembers(MessageNewChatMembers {new_chat_members}) => {
                for member in new_chat_members {
                    create_user(connection, member);
                    println!("Member: {:?}", member);
                }
            }
            MessageKind::LeftChatMember(MessageLeftChatMember{left_chat_member}) => {

                println!("Left Member: {:?}", left_chat_member);
            }

            _ => return Ok(()),
        }

        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;
}