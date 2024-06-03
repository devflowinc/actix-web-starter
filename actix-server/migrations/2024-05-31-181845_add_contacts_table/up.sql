-- Your SQL goes here
CREATE TABLE "contacts" (
	"id" UUID PRIMARY KEY,
	"org_id" UUID NOT NULL,
	"first_name" TEXT NOT NULL,
	"last_name" TEXT NOT NULL,
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id")
);
