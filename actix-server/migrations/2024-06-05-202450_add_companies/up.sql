CREATE TABLE "companies" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
