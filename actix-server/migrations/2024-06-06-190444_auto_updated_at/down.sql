-- This file should undo anything in `up.sql`

ALTER TABLE contacts
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

ALTER TABLE deals
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

ALTER TABLE emails
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

ALTER TABLE links
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

ALTER TABLE phones
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

ALTER TABLE tasks
  DROP COLUMN updated_at,
  DROP COLUMN created_at;

DROP FUNCTION IF EXISTS update_modified_column CASCADE

