-- This file should undo anything in `up.sql`
ALTER TABLE "org_users"
ADD COLUMN "role" INT4 NOT NULL;

