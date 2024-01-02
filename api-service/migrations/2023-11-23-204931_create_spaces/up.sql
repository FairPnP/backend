CREATE TABLE spaces (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL,
  building_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT,
  picture_url VARCHAR,
  max_vehicle_size VARCHAR NOT NULL,
  coverage VARCHAR NOT NULL CHECK (coverage IN ('outdoor', 'outdoor-covered', 'indoor')), 
  height_clearance_cm INTEGER, 
  access_restrictions TEXT,  
  parking_instructions TEXT,  
  last_modified TIMESTAMP NOT NULL DEFAULT (NOW()),
  created_at TIMESTAMP NOT NULL DEFAULT (NOW()),

  CONSTRAINT unique_user_building_name UNIQUE (user_id, building_id, name)
);

CREATE INDEX idx_spaces_user_id ON spaces(user_id);
CREATE INDEX idx_spaces_building_id ON spaces(building_id);

CREATE TRIGGER update_spaces_modtime
BEFORE UPDATE ON spaces
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
