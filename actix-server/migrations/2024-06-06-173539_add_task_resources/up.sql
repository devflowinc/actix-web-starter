-- Your SQL goes here
CREATE TABLE "task_deals" (
	"id" UUID PRIMARY KEY,
	"task_id" UUID NOT NULL,
	"deal_id" UUID NOT NULL,
	FOREIGN KEY ("task_id") REFERENCES "tasks" ("id"),
	FOREIGN KEY ("deal_id") REFERENCES "deals" ("id"),
	UNIQUE ("task_id", "deal_id")
);

CREATE TABLE "task_users" (
	"id" UUID PRIMARY KEY,
	"task_id" UUID NOT NULL,
	"user_id" UUID NOT NULL,
	FOREIGN KEY ("task_id") REFERENCES "tasks" ("id"),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
	UNIQUE ("task_id", "user_id")
);


CREATE TABLE "task_links" (
	"id" UUID PRIMARY KEY,
	"task_id" UUID NOT NULL,
	"link_id" UUID NOT NULL,
	FOREIGN KEY ("task_id") REFERENCES "tasks" ("id"),
	FOREIGN KEY ("link_id") REFERENCES "links" ("id"),
	UNIQUE ("task_id", "link_id")
);
