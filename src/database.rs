use diesel::{dsl::insert_into, PgConnection, Connection, RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::data_types::PgTimestamp;
use diesel::sql_types::Timestamp;
use crate::models;

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {} - err {}", database_url, err))
}

pub fn create_user(conn: &mut PgConnection, user: &teloxide::types::User) {
    use crate::schema::{*};

    let user_id = user.id.0 as i64;
    let username = user.username.clone();
    let first_name = user.first_name.clone();
    let last_name = user.last_name.clone();

    insert_into(telegram_users::dsl::telegram_users)
        .values((
            telegram_users::id.eq(user_id),
            telegram_users::username.eq(username.clone()),
            telegram_users::first_name.eq(first_name.clone()),
            telegram_users::last_name.eq(last_name.clone())
        ))
        .on_conflict(telegram_users::id)
        .do_update()
        .set((
            telegram_users::username.eq(username.clone()),
            telegram_users::first_name.eq(first_name.clone()),
            telegram_users::last_name.eq(last_name.clone()),
            telegram_users::updated_at.eq(diesel::dsl::now)
        ))
        .execute(conn)
        .expect("Error inserting user");
}

pub fn create_group(conn: &mut PgConnection, chat_id: i64) {
    use crate::schema::{*};

    insert_into(telegram_groups::dsl::telegram_groups)
        .values((
            telegram_groups::id.eq(chat_id),
        ))
        .on_conflict(telegram_groups::id)
        .do_nothing()
        .execute(conn)
        .expect("Error inserting group");
}

pub fn insert_user_in_group(conn: &mut PgConnection, group_id: i64, user_id: i64) {
    use crate::schema::{*};

    insert_into(telegram_groups_users::dsl::telegram_groups_users)
        .values((
            telegram_groups_users::group_id.eq(group_id),
            telegram_groups_users::user_id.eq(user_id),
        ))
        .on_conflict((telegram_groups_users::group_id, telegram_groups_users::user_id))
        .do_nothing()
        .execute(conn)
        .expect("Error inserting user in group");
}

pub fn remove_user_from_group(conn: &mut PgConnection, group_id: i64, user_id: i64) {
    use crate::schema::{*};

    diesel::delete(
        telegram_groups_users::dsl::telegram_groups_users
            .filter(telegram_groups_users::group_id.eq(group_id))
            .filter(telegram_groups_users::user_id.eq(user_id)))
        .execute(conn)
        .expect("Error removing user from group");
}

pub fn get_user_id_by_username(conn: &mut PgConnection, username: &str) -> Option<i64> {
    use crate::schema::{*};

    let user_id = telegram_users::dsl::telegram_users
        .filter(telegram_users::username.eq(username))
        .select(telegram_users::id)
        .first::<i64>(conn);

    user_id.ok()
}

pub fn get_user(conn: &mut PgConnection, user_id: i64) -> Option<models::User> {
    use crate::schema::{*};

    let user = telegram_users::dsl::telegram_users
        .filter(telegram_users::id.eq(user_id))
        .first::<models::User>(conn);

    user.ok()
}

pub fn check_user_in_group(conn: &mut PgConnection, group_id: i64, user_id: i64) -> bool {
    use crate::schema::{*};

    let result = telegram_groups_users::dsl::telegram_groups_users
        .filter(telegram_groups_users::group_id.eq(group_id))
        .filter(telegram_groups_users::user_id.eq(user_id))
        .first::<(i64, i64, PgTimestamp)>(conn);

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn create_poll(conn: &mut PgConnection, id: i64, group_id: i64, initiator_id: i64, mentioned_user_id: i64, question: &str, poll_type: &str, start_time: i32, end_time: i32) {
    use crate::schema::{*};

    insert_into(polls::dsl::polls)
        .values((
            polls::id.eq(id),
            polls::group_id.eq(group_id),
            polls::initiator_id.eq(initiator_id),
            polls::mentioned_user_id.eq(mentioned_user_id),
            polls::question.eq(question),
            polls::type_.eq(poll_type),
            polls::start_time.eq(PgTimestamp(start_time as i64)),
            polls::end_time.eq(PgTimestamp(end_time as i64))
        ))
        .execute(conn)
        .expect("Error creating poll");
}

pub fn get_all_active_polls(conn: &mut PgConnection, group_id: i64) -> Vec<models::Poll> {
    use crate::schema::{*};

    let polls = polls::dsl::polls
        .filter(polls::finalized.eq(false))
        .filter(polls::group_id.eq(group_id))
        .load::<models::Poll>(conn)
        .expect("Error loading polls");

    polls
}

pub fn get_poll(conn: &mut PgConnection, poll_id: i64) -> Option<models::Poll> {
    use crate::schema::{*};

    let poll = polls::dsl::polls
        .filter(polls::id.eq(poll_id))
        .first::<models::Poll>(conn);

    poll.ok()
}

pub fn finalize_poll(conn: &mut PgConnection, poll_id: i64) {
    use crate::schema::{*};

    diesel::update(polls::dsl::polls)
        .filter(polls::id.eq(poll_id))
        .set(polls::finalized.eq(true))
        .execute(conn)
        .expect("Error finalizing poll");
}
