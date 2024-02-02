CREATE TABLE user_summaries (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    total_reviews INTEGER NOT NULL DEFAULT 0,
    average_stars INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_summaries_user_id ON user_summaries(user_id);

CREATE TRIGGER update_user_summaries_modtime
BEFORE UPDATE ON user_summaries
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
