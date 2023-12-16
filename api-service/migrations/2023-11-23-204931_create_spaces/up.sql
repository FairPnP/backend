CREATE TABLE spaces (
  id SERIAL PRIMARY KEY,
  building_id integer NOT NULL,
  user_id UUID NOT NULL,
  name varchar NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now()),

  CONSTRAINT unique_user_building_name UNIQUE (user_id, building_id, name)
);

CREATE INDEX idx_spaces_user_id ON spaces(user_id);
CREATE INDEX idx_spaces_building_id ON spaces(building_id);

CREATE TRIGGER update_spaces_modtime
BEFORE UPDATE ON spaces
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
