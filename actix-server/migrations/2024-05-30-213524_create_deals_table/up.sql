-- Your SQL goes here
CREATE TABLE "deals" (
	"id" UUID PRIMARY KEY,
	"name" TEXT,
	"org_id" UUID NOT NULL,
	"size" REAL,
	"active" BOOLEAN NOT NULL DEFAULT FALSE,
	FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
