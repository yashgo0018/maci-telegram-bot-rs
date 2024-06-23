use diesel::PgConnection;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use crate::database::get_user;

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
            "make_admin" => Some(AvailableVotingTypes::MakeAdmin),
            "remove_admin" => Some(AvailableVotingTypes::RemoveAdmin),
            "kick_user" => Some(AvailableVotingTypes::KickUser),
            "ban_user" => Some(AvailableVotingTypes::BanUser),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            AvailableVotingTypes::MakeAdmin => "make_admin",
            AvailableVotingTypes::RemoveAdmin => "remove_admin",
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

    async fn create_poll(&self, conn: &mut PgConnection, bot: &Bot, chat_id: i64, mentioned_user_id: i64) -> ResponseResult<()> {
        let user = get_user(conn, mentioned_user_id).unwrap();
        let mut name = user.first_name;
        if let Some(last_name) = user.last_name {
            name.push_str(" ");
            name.push_str(&last_name);
        }
        if let Some(username) = user.username {
            name.push_str(" (@");
            name.push_str(&username);
            name.push_str(")");
        }

        let poll_question = format!("Should {} be {}?", name, self.to_title_object());
        let poll_options = vec!["Yes".to_string(), "No".to_string()];

        bot.send_poll(ChatId(chat_id), poll_question, poll_options)
            .is_anonymous(true)
            .allows_multiple_answers(false)
            .await?;

        Ok(())
    }
}


pub async fn handle_vote_command(conn: &mut PgConnection, bot: &Bot, chat_id: i64, vote_type: &str, mentioned_user_id: i64) -> ResponseResult<()> {
    let vote_type = AvailableVotingTypes::from_str(vote_type);
    if vote_type.is_none() {
        bot.send_message(ChatId(chat_id),  "Invalid voting type").await?;
        return Ok(());
    }
    let vote_type = vote_type.unwrap();

    // vote_type.handle_vote_command(bot, chat_id, mentioned_user_id).await?;
    vote_type.create_poll(conn, bot, chat_id, mentioned_user_id).await?;

    println!("Vote Type: {:?}", vote_type);
    println!("Mention User Id: {:?}", mentioned_user_id);

    Ok(())
}