-- Drop the trigger
DROP TRIGGER IF EXISTS update_spaces_modtime ON spaces;

-- Drop the indexes
DROP INDEX IF EXISTS idx_spaces_user_id;
DROP INDEX IF EXISTS idx_spaces_building_id;

-- Drop the table
DROP TABLE IF EXISTS spaces;
