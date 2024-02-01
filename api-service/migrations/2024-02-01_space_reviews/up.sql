CREATE TABLE space_reviews (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    space_id INTEGER NOT NULL,
    message TEXT NOT NULL,
    stars INTEGER NOT NULL CHECK (stars BETWEEN 1 AND 5),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT space_reviews_unique_user_space UNIQUE (user_id, space_id)
);

CREATE INDEX idx_space_reviews_user_id ON space_reviews(user_id);
CREATE INDEX idx_space_reviews_space_id ON space_reviews(space_id);

CREATE TRIGGER update_space_reviews_modtime
BEFORE UPDATE ON space_reviews
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
