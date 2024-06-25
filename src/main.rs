mod models;
mod schema;
mod database;
mod bot;
mod voting_api;
mod utils;

use dotenvy::dotenv;
use teloxide::prelude::*;
use teloxide::types::{MediaText, MessageKind, MessageLeftChatMember, MessageNewChatMembers};
use crate::bot::handle_vote_command;

use crate::database::{db_create_group, db_create_user, db_establish_connection, db_get_user_id_by_username, db_insert_user_in_group, db_remove_user_from_group};

#[tokio::main]
async fn main() {

    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let connection = &mut db_establish_connection();

        // if the message is a command "/vote vote_type mention" then continue else return
        // print the chat id
        let mut initiator_id = 0;
        let chat_id = msg.chat.id.0;
        db_create_group(connection, chat_id);

        if let Some(sender) = msg.from() {
            initiator_id = sender.id.0 as i64;
            // create user
            db_create_user(connection, sender);
            // insert user in the group
            db_insert_user_in_group(connection, chat_id, initiator_id);
        }

        println!("Message: {:?}", msg);

        match &msg.kind {
            MessageKind::NewChatMembers(MessageNewChatMembers {new_chat_members}) => {
                for member in new_chat_members {
                    let member_id = member.id.0 as i64;
                    // create user
                    db_create_user(connection, member);
                    // insert user in the group
                    db_insert_user_in_group(connection, chat_id, member_id);
                }
            }
            MessageKind::LeftChatMember(MessageLeftChatMember{left_chat_member}) => {
                let member_id = left_chat_member.id.0 as i64;
                // create user
                db_create_user(connection, left_chat_member);
                // insert user in the group
                db_insert_user_in_group(connection, chat_id, member_id);
                // remove user from the group
                db_remove_user_from_group(connection, chat_id, member_id);
            }
            MessageKind::Common(message) => {
                match &message.media_kind {
                    teloxide::types::MediaKind::Text(MediaText{text, entities}) => {
                        println!("Text: {:?}", text);
                        // check messages for the following format
                        // /vote vote_type mention
                        // if the message is in the correct format then continue else return
                        // print the vote_type and mention

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

                        let vote_type = &text[entities[0].length+1..entities[1].offset-1];

                        let mentioned_user_id: i64;

                        match &entities[1].kind {
                            teloxide::types::MessageEntityKind::Mention => {
                                let mention = &text[entities[1].offset+1..];
                                let user_id = db_get_user_id_by_username(connection, mention);
                                if let Some(user_id) = user_id {
                                    mentioned_user_id = user_id;
                                } else {
                                    bot.send_message(ChatId(chat_id),  "User not found").await?;
                                    return Ok(());
                                }
                            }
                            teloxide::types::MessageEntityKind::TextMention {user} => {
                                mentioned_user_id = user.id.0 as i64;
                                // create user
                                db_create_user(connection, user);
                            }
                            _ => {
                                return Ok(());
                            }
                        }

                        // check if the user is in the group
                        let user = bot.get_chat_member(ChatId(chat_id), UserId(mentioned_user_id as u64)).await?;
                        if user.kind == teloxide::types::ChatMemberKind::Left {
                            bot.send_message(ChatId(chat_id),  "User not in group").await?;
                            return Ok(());
                        }

                        // insert user in the group
                        db_insert_user_in_group(connection, chat_id, mentioned_user_id);

                        println!("user: {:?}", user);

                        handle_vote_command(connection, &bot, chat_id, vote_type, initiator_id, mentioned_user_id).await?;
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
