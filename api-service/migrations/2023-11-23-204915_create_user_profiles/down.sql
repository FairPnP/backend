-- Drop the trigger
DROP TRIGGER IF EXISTS update_user_profile_modtime ON user_profiles;

-- Drop the index
DROP INDEX IF EXISTS idx_user_profiles_on_user_id;

-- Drop the table
DROP TABLE IF EXISTS user_profiles;
