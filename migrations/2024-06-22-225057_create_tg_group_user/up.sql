-- Your SQL goes here
CREATE TABLE telegram_groups_users (
    group_id BIGINT,
    user_id BIGINT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (group_id, user_id),
    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES telegram_groups(id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES telegram_users(id)
);
