CREATE TABLE space_favorites (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    space_id integer NOT NULL,
    created_at timestamp NOT NULL DEFAULT (now()),

    -- Unique constraint to ensure a user can't favorite the same space more than once
    CONSTRAINT unique_user_space UNIQUE (user_id, space_id)
);

-- Index for querying all favorite spaces for a specific user_id
CREATE INDEX idx_space_favorites_user_id ON space_favorites(user_id);
