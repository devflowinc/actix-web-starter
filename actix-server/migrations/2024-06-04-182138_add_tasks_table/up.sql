-- Your SQL goes here
CREATE TABLE "tasks" (
	"id" UUID PRIMARY KEY,
	"deadline" TIMESTAMP,
	"description" TEXT,
	"contact_id" UUID,
	"org_id" UUID NOT NULL,
	FOREIGN KEY ("contact_id") REFERENCES "contacts" ("id"),
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id")
);
