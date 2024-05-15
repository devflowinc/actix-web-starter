CREATE TABLE "plans"(
	"id" UUID NOT NULL PRIMARY KEY,
	"num_users" INTEGER NOT NULL,
	"price_per_month" INTEGER NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "orgs"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "org_plans"(
	"id" UUID NOT NULL PRIMARY KEY,
	"org_id" UUID NOT NULL,
	"plan_id" UUID NOT NULL,
	"next_billing_date" TIMESTAMP NOT NULL,
	"start_date" TIMESTAMP NOT NULL,
	"end_date" TIMESTAMP,
	FOREIGN KEY ("org_id") REFERENCES "orgs"("id"),
	FOREIGN KEY ("plan_id") REFERENCES "plans"("id")
);

CREATE TABLE "org_users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"org_id" UUID NOT NULL,
	"role" INTEGER NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id"),
	FOREIGN KEY ("org_id") REFERENCES "orgs"("id")
);


