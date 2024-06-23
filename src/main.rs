mod models;
mod schema;
mod database;
mod bot;

use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::types::{MediaText, MessageCommon, MessageKind, MessageLeftChatMember, MessageNewChatMembers};
use crate::bot::handle_vote_command;

use crate::database::{create_group, create_user, establish_connection, get_user_id_by_username, insert_user_in_group, remove_user_from_group};

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

        println!("Message: {:?}", msg);

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
            MessageKind::Common(message) => {
                match &message.media_kind {
                    teloxide::types::MediaKind::Text(MediaText{text, entities}) => {
                        println!("Text: {:?}", text);
                        // check messages for the following format
                        // /vote option mention
                        // if the message is in the correct format then continue else return
                        // print the option and mention

                        if entities.len() != 2 {
                            return Ok(());
                        }

                        if entities[0].kind != teloxide::types::MessageEntityKind::BotCommand && entities[0].offset != 0{
                            return Ok(());
                        }

                        if text.len() != entities[1].offset + entities[1].length {
                            return Ok(());
                        }

                        let command = &text[entities[0].offset..entities[0].offset + entities[0].length];
                        if command != "/vote" {
                            return Ok(());
                        }

                        let option = &text[entities[0].length+1..entities[1].offset-1];

                        let mentioned_user_id: i64;

                        match &entities[1].kind {
                            teloxide::types::MessageEntityKind::Mention => {
                                let mention = &text[entities[1].offset+1..];
                                let user_id = get_user_id_by_username(connection, mention);
                                if let Some(user_id) = user_id {
                                    mentioned_user_id = user_id;
                                } else {
                                    bot.send_message(ChatId(chat_id),  "User not found").await?;
                                    return Ok(());
                                }
                            }
                            teloxide::types::MessageEntityKind::TextMention {user} => {
                                create_user(connection, user);
                                mentioned_user_id = user.id.0 as i64;
                            }
                            _ => {
                                return Ok(());
                            }
                        }

                        // check if the user is in the group
                        let user = bot.get_chat_member(ChatId(chat_id), UserId(mentioned_user_id as u64)).await?;
                        // if user.is_none() {
                        //     bot.send_message(ChatId(chat_id),  "User not in group").await?;
                        //     return Ok(());
                        // }
                        if user.kind == teloxide::types::ChatMemberKind::Left {
                            bot.send_message(ChatId(chat_id),  "User not in group").await?;
                            return Ok(());
                        }

                        println!("user: {:?}", user);

                        handle_vote_command(option, mentioned_user_id);
                    }
                    _ => {}
                }
            }

            _ => return Ok(()),
        };

        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;
}
