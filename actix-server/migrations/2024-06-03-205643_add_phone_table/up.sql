-- Your SQL goes here
CREATE TABLE "phones" (
	"id" UUID PRIMARY KEY,
	"number" TEXT NOT NULL,
	"org_id" UUID NOT NULL,
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
