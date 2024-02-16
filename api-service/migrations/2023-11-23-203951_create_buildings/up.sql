CREATE TABLE buildings (
  id SERIAL PRIMARY KEY,
  name varchar NOT NULL,
  place_id varchar NOT NULL UNIQUE,
  latitude DECIMAL(9, 6) NOT NULL,
  longitude DECIMAL(9, 6) NOT NULL,
  street_number varchar NOT NULL,
  street_name varchar NOT NULL,
  city varchar NOT NULL,
  state varchar NOT NULL,
  postal_code varchar NOT NULL,
  country varchar NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX idx_building_place_id ON buildings(place_id);

CREATE TRIGGER update_building_modtime
BEFORE UPDATE ON buildings
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
