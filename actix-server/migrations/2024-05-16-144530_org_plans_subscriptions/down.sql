-- This file should undo anything in `up.sql`

ALTER TABLE "users" DROP COLUMN "first_name";
ALTER TABLE "users" DROP COLUMN "last_name";
ALTER TABLE "users" ADD COLUMN "name" TEXT;

DROP TABLE IF EXISTS "org_users_perms";
DROP TABLE IF EXISTS "org_users";
DROP TABLE IF EXISTS "subscriptions";
DROP TABLE IF EXISTS "plans";
DROP TABLE IF EXISTS "orgs";

DROP TYPE PERM;
