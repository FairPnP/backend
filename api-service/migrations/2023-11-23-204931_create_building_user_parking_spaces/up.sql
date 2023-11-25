CREATE TABLE building_user_parking_spaces (
  id SERIAL PRIMARY KEY,
  building_user_id integer NOT NULL,
  name varchar NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now())
);

-- ALTER TABLE building_user_parking_spaces ADD FOREIGN KEY (building_user_id) REFERENCES building_users (id);

CREATE INDEX idx_building_user_parking_spaces_on_building_user_id ON building_user_parking_spaces(building_user_id);
CREATE UNIQUE INDEX idx_building_user_parking_spaces_on_building_user_id_and_name ON building_user_parking_spaces(building_user_id, name);

CREATE TRIGGER update_building_modtime
BEFORE UPDATE ON building_user_parking_spaces
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
