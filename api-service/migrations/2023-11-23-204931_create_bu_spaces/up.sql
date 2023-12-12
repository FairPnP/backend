CREATE TABLE bu_spaces (
  id SERIAL PRIMARY KEY,
  building_id integer NOT NULL,
  user_id UUID NOT NULL,
  name varchar NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now()),

  CONSTRAINT unique_user_building_name UNIQUE (user_id, building_id, name)
);

CREATE INDEX idx_bu_spaces_user_id ON bu_spaces(user_id);
CREATE INDEX idx_bu_spaces_building_id ON bu_spaces(building_id);

CREATE TRIGGER update_bu_spaces_modtime
BEFORE UPDATE ON bu_spaces
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
