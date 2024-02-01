CREATE TABLE space_summaries (
    id SERIAL PRIMARY KEY,
    host_user_id UUID NOT NULL,
    space_id INTEGER NOT NULL UNIQUE,
    total_reviews INTEGER NOT NULL DEFAULT 0,
    average_stars INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_space_summaries_host_user_id ON space_summaries(host_user_id);
CREATE INDEX idx_space_summaries_space_id ON space_summaries(space_id);

CREATE TRIGGER update_space_summaries_modtime
BEFORE UPDATE ON space_summaries
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
