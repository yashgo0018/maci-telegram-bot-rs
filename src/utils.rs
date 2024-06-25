pub fn get_poll_link(poll_id: i64) -> String {
    format!("https://bot.zk-voting.com/vote/{}", poll_id)
}
