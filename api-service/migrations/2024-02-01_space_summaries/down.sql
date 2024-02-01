DROP TRIGGER IF EXISTS update_space_summaries_modtime ON space_summaries;
DROP INDEX IF EXISTS idx_space_summaries_space_id;
DROP INDEX IF EXISTS idx_space_summaries_host_user_id;
DROP TABLE IF EXISTS space_summaries;
