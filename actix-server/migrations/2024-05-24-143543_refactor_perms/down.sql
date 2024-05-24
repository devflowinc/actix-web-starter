-- This file should undo anything in `up.sql`
ALTER TABLE "org_users"
DROP COLUMN "role";

DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'perm') THEN
        CREATE TYPE perm AS ENUM ('subscription');
    END IF;
END $$;


CREATE TABLE "org_users_perms"(
	"org_user_id" UUID NOT NULL PRIMARY KEY,
	"perm" PERM,
	"has" BOOL NOT NULL,
	FOREIGN KEY ("org_user_id") REFERENCES "org_users"("id") ON DELETE CASCADE
);

