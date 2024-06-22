use diesel::prelude::*;
use diesel::data_types::PgTimestamp;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::telegram_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}
