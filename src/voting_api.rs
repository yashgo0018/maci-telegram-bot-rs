use crate::bot::AvailableVotingTypes;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreatePollRequest {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub vote_type: String,
    #[serde(rename = "telegramChatId")]
    pub telegram_chat_id: String,
    pub duration: i64
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreatePollResponse {
    #[serde(rename = "pollId")]
    pub id: i64,
    #[serde(rename = "startTime")]
    pub start_time: i32,
    #[serde(rename = "endTime")]
    pub end_time: i32,
    pub message: String
}

pub async fn call_create_poll_api(name: &str, description: &str, vote_type: &AvailableVotingTypes, telegram_chat_id: i64) -> Result<CreatePollResponse, reqwest::Error>{
    let duration = 86400;
    let vote_type = vote_type.to_str();
    let endpoint = std::env::var("MACI_API_SERVER").expect("MACI_API_SERVER must be set");
    let url = format!("{endpoint}/create-poll");
    // make a post request to the api server
    let client = reqwest::Client::new();
    let body = CreatePollRequest {
        name: name.to_string(),
        description: description.to_string(),
        vote_type: vote_type.to_string(),
        telegram_chat_id: telegram_chat_id.to_string(),
        duration
    };

    let res = client.post(&url)
        .json(&body)
        .send()
        .await?
        .json::<CreatePollResponse>()
        .await
        .expect("Error creating poll");

    println!("poll id: {:?}", res.id);

    Ok(res)
}
