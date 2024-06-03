-- Your SQL goes here
CREATE TABLE "links" (
	"id" UUID PRIMARY KEY,
	"link" TEXT NOT NULL,
	"org_id" UUID NOT NULL,
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
