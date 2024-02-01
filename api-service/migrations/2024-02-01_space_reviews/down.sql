DROP TRIGGER IF EXISTS update_space_reviews_modtime ON space_reviews;
DROP INDEX IF EXISTS idx_space_reviews_space_id;
DROP INDEX IF EXISTS idx_space_reviews_user_id;
DROP TABLE IF EXISTS space_reviews;
