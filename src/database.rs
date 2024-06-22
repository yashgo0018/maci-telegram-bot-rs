use diesel::{dsl::insert_into, PgConnection, Connection, RunQueryDsl, ExpressionMethods};

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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