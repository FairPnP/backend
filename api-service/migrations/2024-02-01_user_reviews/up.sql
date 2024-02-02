CREATE TABLE user_reviews (
    id SERIAL PRIMARY KEY,
    from_user_id UUID NOT NULL,
    to_user_id UUID NOT NULL,
    message TEXT NOT NULL,
    stars INTEGER NOT NULL CHECK (stars BETWEEN 1 AND 5),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT user_reviews_unique_user_user UNIQUE (from_user_id, to_user_id)
);

CREATE INDEX idx_user_reviews_from_user_id ON user_reviews(from_user_id);
CREATE INDEX idx_user_reviews_to_user_id ON user_reviews(to_user_id);
CREATE INDEX idx_user_user_reviews ON user_reviews(from_user_id, to_user_id);

CREATE TRIGGER update_user_reviews_modtime
BEFORE UPDATE ON user_reviews
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
