CREATE TABLE "notes" (
    "id" UUID PRIMARY KEY,
    "title" TEXT NOT NULL,
    "body" TEXT NOT NULL,
    "org_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY ("org_id") REFERENCES "orgs" ("id") ON DELETE CASCADE
);
