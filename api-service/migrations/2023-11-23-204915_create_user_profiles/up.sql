CREATE TABLE user_profiles (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL UNIQUE,
  name varchar NOT NULL,
  avatar_url varchar,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX idx_user_profiles_on_user_id ON user_profiles(user_id);

CREATE TRIGGER update_building_modtime
BEFORE UPDATE ON user_profiles
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
