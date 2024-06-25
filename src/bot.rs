use diesel::PgConnection;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use crate::database::{db_check_if_active_poll_exists, db_create_poll, db_get_user};
use crate::utils::get_poll_link;
use crate::voting_api::call_create_poll_api;

#[derive(Debug)]
pub enum AvailableVotingTypes {
    MakeAdmin,
    RemoveAdmin,
    KickUser,
    BanUser
}

impl AvailableVotingTypes {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "addAdmin" => Some(AvailableVotingTypes::MakeAdmin),
            "removeAdmin" => Some(AvailableVotingTypes::RemoveAdmin),
            "kick_user" => Some(AvailableVotingTypes::KickUser),
            "ban_user" => Some(AvailableVotingTypes::BanUser),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            AvailableVotingTypes::MakeAdmin => "addAdmin",
            AvailableVotingTypes::RemoveAdmin => "removeAdmin",
            AvailableVotingTypes::KickUser => "kick_user",
            AvailableVotingTypes::BanUser => "ban_user",
        }
    }

    pub fn to_title_object(&self) -> &str {
        match self {
            AvailableVotingTypes::MakeAdmin => "given admin role",
            AvailableVotingTypes::RemoveAdmin => "removed from admin role",
            AvailableVotingTypes::KickUser => "kicked out of the group",
            AvailableVotingTypes::BanUser => "banned from the group",
        }
    }

    async fn create_poll(&self, conn: &mut PgConnection, bot: &Bot, chat_id: i64, initiator_id: i64, mentioned_user_id: i64) -> ResponseResult<()> {
        let user = db_get_user(conn, mentioned_user_id).unwrap();

        // check if an active poll exists for the same thing
        if db_check_if_active_poll_exists(conn, chat_id, mentioned_user_id, self.to_str()) {
            bot.send_message(ChatId(chat_id), "A vote for the same action is already in progress").await?;
            return Ok(());
        }

        // format user's name to be displayed in the poll
        let mut name = user.first_name;
        if let Some(last_name) = user.last_name {
            name.push_str(format!(" {last_name}").as_str());
        }
        if let Some(username) = user.username {
            name.push_str(format!(" (@{username})").as_str());
        }

        // create poll question and options
        let poll_question = format!("Should {} be {}?", name, self.to_title_object());

        // create poll using the http api
        let poll_created_response = call_create_poll_api(
            poll_question.as_str(),
            "Vote to make user admin",
            self,
            chat_id
        ).await.expect("Error creating poll");

        db_create_poll(
            conn,
            chat_id,
            poll_created_response.id,
            initiator_id,
            mentioned_user_id,
            &poll_question,
            self.to_str(),
            poll_created_response.start_time,
            poll_created_response.end_time
        );

        let poll_link = get_poll_link(poll_created_response.id);

        // send the link of the poll to the group
        let message = bot.send_message(ChatId(chat_id), format!("Vote for '{poll_question}' at {poll_link}")).await?;
        bot.pin_chat_message(ChatId(chat_id), message.id).await?;

        Ok(())
    }
}

pub async fn handle_vote_command(conn: &mut PgConnection, bot: &Bot, chat_id: i64, vote_type: &str, initiator_id: i64, mentioned_user_id: i64) -> ResponseResult<()> {
    let vote_type = AvailableVotingTypes::from_str(vote_type);
    if vote_type.is_none() {
        bot.send_message(ChatId(chat_id),  "Invalid voting type").await?;
        return Ok(());
    }
    let vote_type = vote_type.unwrap();

    vote_type.create_poll(conn, bot, chat_id, initiator_id, mentioned_user_id).await?;

    println!("Vote Type: {:?}", vote_type);
    println!("Mention User Id: {:?}", mentioned_user_id);

    Ok(())
}