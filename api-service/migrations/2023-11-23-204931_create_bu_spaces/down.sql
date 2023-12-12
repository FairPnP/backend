-- Drop the trigger
DROP TRIGGER IF EXISTS update_bu_spaces_modtime ON bu_spaces;

-- Drop the indexes
DROP INDEX IF EXISTS idx_bu_spaces_user_id;
DROP INDEX IF EXISTS idx_bu_spaces_building_id;

-- Drop the table
DROP TABLE IF EXISTS bu_spaces;
