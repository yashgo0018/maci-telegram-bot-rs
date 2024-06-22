-- Your SQL goes here

ALTER TABLE telegram_users
ADD COLUMN first_name VARCHAR(100) NOT NULL DEFAULT '';

ALTER TABLE telegram_users
ADD COLUMN last_name VARCHAR(100);