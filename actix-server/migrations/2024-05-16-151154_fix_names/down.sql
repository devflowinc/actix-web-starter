-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "name";
ALTER TABLE "users" ADD COLUMN "first_name" TEXT;
ALTER TABLE "users" ADD COLUMN "last_name" TEXT;
