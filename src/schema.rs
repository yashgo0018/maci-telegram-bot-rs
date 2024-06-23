// @generated automatically by Diesel CLI.

diesel::table! {
    User (id) {
        id -> Text,
        secretKey -> Text,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    telegram_groups (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    telegram_groups_users (group_id, user_id) {
        group_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    telegram_users (id) {
        id -> Int8,
        username -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(telegram_groups_users -> telegram_groups (group_id));
diesel::joinable!(telegram_groups_users -> telegram_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    User,
    _prisma_migrations,
    telegram_groups,
    telegram_groups_users,
    telegram_users,
);
