-- This file should undo anything in `up.sql`

ALTER TABLE telegram_users DROP COLUMN IF EXISTS first_name;
ALTER TABLE telegram_users DROP COLUMN IF EXISTS last_name;
