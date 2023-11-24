CREATE TABLE "user_ratings" (
  "id" integer PRIMARY KEY,
  "user_id" UUID NOT NULL,
  "rated_by_user_id" UUID NOT NULL,
  "star_rating" integer NOT NULL,
  "last_modified" timestamp NOT NULL DEFAULT (now()),
  "created_at" timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX idx_user_ratings_on_user_id ON user_ratings(user_id);
CREATE INDEX idx_user_ratings_on_rated_by_user_id ON user_ratings(rated_by_user_id);
