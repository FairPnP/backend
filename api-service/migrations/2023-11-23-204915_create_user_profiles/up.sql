CREATE TABLE "user_profiles" (
  "id" integer PRIMARY KEY,
  "user_id" UUID NOT NULL,
  "name" varchar NOT NULL,
  "avatar_url" varchar,
  "last_modified" timestamp NOT NULL DEFAULT (now()),
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE UNIQUE INDEX idx_user_profiles_on_user_id ON user_profiles(user_id);
