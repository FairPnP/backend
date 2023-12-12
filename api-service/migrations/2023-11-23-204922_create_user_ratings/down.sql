-- Drop the trigger
DROP TRIGGER IF EXISTS update_user_rating_modtime ON user_ratings;

-- Drop the indexes
DROP INDEX IF EXISTS idx_user_ratings_on_user_id;
DROP INDEX IF EXISTS idx_user_ratings_on_rated_by_user_id;

-- Drop the table
DROP TABLE IF EXISTS user_ratings;
