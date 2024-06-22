-- This file should undo anything in `up.sql`

ALTER TABLE telegram_users ALTER COLUMN username SET NOT NULL;
