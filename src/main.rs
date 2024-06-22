mod models;
mod schema;

use diesel::dsl::insert_into;
use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{MessageKind, MessageLeftChatMember, MessageNewChatMembers};

pub fn establish_connection() -> PgConnection {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &PgConnection, user: &teloxide::types::User) {
    use self::schema::{*, telegram_users::dsl::*};

    let user_id = user.id.0 as i64;
    let username = user.username.clone();
    let first_name = user.first_name.clone();
    let last_name = user.last_name.clone();

    insert_into(telegram_users)
        .values((
            telegram_users::id.eq(user_id),
            telegram_users::username.eq(username),
            telegram_users::first_name.eq(first_name),
            telegram_users::last_name.eq(last_name)
        ))
        .on_conflict(telegram_users::id)
        .do_update()
        .set((
            telegram_users::username.eq(username),
            telegram_users::first_name.eq(first_name),
            telegram_users::last_name.eq(last_name),
            telegram_users::updated_at.eq(diesel::dsl::now)
        ))
        .execute(conn)
        .expect("Error inserting user");
}

// pub fn add_user_to_chat(conn: &PgConnection, user_id: i64, chat_id: i64) {
//     use self::schema::{*, chat_users::dsl::*};
//
//     insert_into(chat_users)
//         .values((
//             chat_users::user_id.eq(user_id),
//             chat_users::chat_id.eq(chat_id)
//         ))
//         .on_conflict((chat_users::user_id, chat_users::chat_id))
//         .do_nothing()
//         .execute(conn)
//         .expect("Error inserting user to chat");
//
// }

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