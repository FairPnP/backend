DROP TRIGGER IF EXISTS update_user_summaries_modtime ON user_summaries;
DROP INDEX IF EXISTS idx_user_summaries_user_id;
DROP TABLE IF EXISTS user_summaries;
