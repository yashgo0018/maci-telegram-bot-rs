-- Your SQL goes here

CREATE TABLE polls (
    id BIGINT PRIMARY KEY,
    question TEXT NOT NULL,
    type TEXT NOT NULL,
    group_id BIGINT NOT NULL,
    initiator_id BIGINT NOT NULL,
    mentioned_user_id BIGINT,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    finalized BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (initiator_id) REFERENCES telegram_users(id),
    FOREIGN KEY (mentioned_user_id) REFERENCES telegram_users(id)
);
