DROP TRIGGER IF EXISTS update_user_notif_tokens_modtime ON user_notif_tokens;
DROP INDEX IF EXISTS idx_user_notif_tokens_user_id;
DROP TABLE IF EXISTS user_notif_tokens;
