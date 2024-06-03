-- Your SQL goes here
CREATE TABLE "emails" (
	"id" UUID PRIMARY KEY,
	"email" TEXT NOT NULL,
	"org_id" UUID NOT NULL,
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
