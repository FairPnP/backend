DROP TRIGGER IF EXISTS update_user_reviews_modtime ON user_reviews;
DROP INDEX IF EXISTS idx_user_reviews_user_id;
DROP TABLE IF EXISTS user_reviews;
