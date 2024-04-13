-- This file should undo anything in `up.sql`
ALTER TABLE "posts" DROP COLUMN "title";
ALTER TABLE "posts" ADD COLUMN "title" VARCHAR(255) NOT NULL;

