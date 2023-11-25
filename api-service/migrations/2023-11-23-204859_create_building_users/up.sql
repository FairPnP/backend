CREATE TABLE building_users (
  id SERIAL PRIMARY KEY,
  building_id integer NOT NULL,
  user_id UUID NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now())
);

-- ALTER TABLE building_users ADD FOREIGN KEY (building_id) REFERENCES buildings (id);

CREATE INDEX idx_building_users_on_building_id ON building_users(building_id);
CREATE INDEX idx_building_users_on_user_id ON building_users(user_id);
CREATE UNIQUE INDEX idx_building_users_on_building_id_and_user_id ON building_users(building_id, user_id);

CREATE TRIGGER update_building_modtime
BEFORE UPDATE ON building_users
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
