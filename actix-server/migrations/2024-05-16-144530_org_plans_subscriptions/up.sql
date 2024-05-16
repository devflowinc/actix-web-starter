-- Your SQL goes here

ALTER TABLE "users" DROP COLUMN "name";
ALTER TABLE "users" ADD COLUMN "first_name" TEXT;
ALTER TABLE "users" ADD COLUMN "last_name" TEXT;

CREATE TYPE PERM as ENUM ('subscription');

CREATE TABLE "orgs"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "plans"(
	"id" UUID NOT NULL PRIMARY KEY,
	"stripe_id" TEXT NOT NULL UNIQUE,
	"num_users" INT4 NOT NULL,
	"num_deals" INT4 NOT NULL,
	"price_per_month" INTEGER NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "subscriptions"(
	"id" UUID NOT NULL PRIMARY KEY,
	"stripe_id" TEXT NOT NULL,
	"org_id" UUID NOT NULL,
	"plan_id" UUID NOT NULL,
	"stripe_plan_id" TEXT NOT NULL,
	"next_billing_date" TIMESTAMP NOT NULL,
	"start_date" TIMESTAMP NOT NULL,
	"end_date" TIMESTAMP,
	FOREIGN KEY ("org_id") REFERENCES "orgs"("id") ON DELETE CASCADE,
	FOREIGN KEY ("plan_id") REFERENCES "plans"("id"),
	FOREIGN KEY ("stripe_plan_id") REFERENCES "plans"("stripe_id")
);

CREATE TABLE "org_users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"org_id" UUID NOT NULL,
	"role" INT4 NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id"),
	FOREIGN KEY ("org_id") REFERENCES "orgs"("id") ON DELETE CASCADE
);

CREATE TABLE "org_users_perms"(
	"org_user_id" UUID NOT NULL PRIMARY KEY,
	"perm" PERM,
	"has" BOOL NOT NULL,
	FOREIGN KEY ("org_user_id") REFERENCES "org_users"("id") ON DELETE CASCADE
);

