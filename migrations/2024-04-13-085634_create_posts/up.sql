-- Your SQL goes here
ALTER TABLE "posts" DROP COLUMN "title";
ALTER TABLE "posts" ADD COLUMN "title" VARCHAR NOT NULL;

