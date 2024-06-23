mod models;
mod schema;
mod database;

use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::types::{MessageKind, MessageLeftChatMember, MessageNewChatMembers};

use crate::database::{create_group, create_user, establish_connection, insert_user_in_group, remove_user_from_group};

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
        create_group(connection, chat_id);
        println!("Chat ID: {}", chat_id);

        if let Some(sender) = msg.from() {
            create_user(connection, sender);
            println!("Sender ID: {:?}", sender);
        }

        match &msg.kind {
            MessageKind::NewChatMembers(MessageNewChatMembers {new_chat_members}) => {
                for member in new_chat_members {
                    create_user(connection, member);
                    let member_id = member.id.0 as i64;
                    insert_user_in_group(connection, chat_id, member_id);
                    println!("Member: {:?}", member);
                }
            }
            MessageKind::LeftChatMember(MessageLeftChatMember{left_chat_member}) => {
                let member_id = left_chat_member.id.0 as i64;
                remove_user_from_group(connection, chat_id, member_id);
                println!("Left Member: {:?}", left_chat_member);
            }

            _ => return Ok(()),
        }

        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;
}
