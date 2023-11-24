CREATE TABLE "buildings" (
  "id" integer PRIMARY KEY,
  "name" varchar NOT NULL,
  "place_id" varchar NOT NULL,
  "latitude" DECIMAL(9, 6) NOT NULL,
  "longitude" DECIMAL(9, 6) NOT NULL,
  "last_modified" timestamp NOT NULL DEFAULT (now()),
  "created_at" timestamp NOT NULL DEFAULT (now())
);
