-- Your SQL goes here
ALTER TABLE "users" DROP COLUMN "first_name";
ALTER TABLE "users" DROP COLUMN "last_name";
ALTER TABLE "users" ADD COLUMN "name" TEXT;
