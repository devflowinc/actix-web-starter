-- This file should undo anything in `up.sql`

ALTER TABLE "task_deals"
DROP COLUMN "created_at",
DROP COLUMN "updated_at";

ALTER TABLE "task_users"
DROP COLUMN "created_at",
DROP COLUMN "updated_at";

ALTER TABLE "task_links"
DROP COLUMN "created_at",
DROP COLUMN "updated_at";

DROP TRIGGER update_task_deals_updated_at ON task_deals;
DROP TRIGGER update_task_links_updated_at ON task_links;
DROP TRIGGER update_task_users_updated_at ON task_users;
