ALTER TABLE "org_users"
ADD CONSTRAINT unique_user_org
UNIQUE ("user_id", "org_id");
