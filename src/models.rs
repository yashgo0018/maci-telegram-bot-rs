use diesel::prelude::*;
use diesel::data_types::PgTimestamp;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::telegram_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Poll {
    pub id: i64,
    pub question: String,
    pub type_: String,
    pub group_id: i64,
    pub initiator_id: i64,
    pub mentioned_user_id: Option<i64>,
    pub start_time: PgTimestamp,
    pub end_time: PgTimestamp,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub finalized: bool,
}
