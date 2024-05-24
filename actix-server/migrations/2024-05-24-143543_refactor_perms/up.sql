-- Your SQL goes here
drop table org_users_perms;

DO $$ 
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'perm') THEN
        DROP TYPE perm;
    END IF;
END $$;

ALTER TABLE "org_users"
ADD COLUMN "role" INT4 NOT NULL;
