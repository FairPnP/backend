-- Drop the trigger
DROP TRIGGER IF EXISTS update_building_modtime ON buildings;

-- Drop the index
DROP INDEX IF EXISTS idx_building_place_id;

-- Drop the table
DROP TABLE IF EXISTS buildings;
