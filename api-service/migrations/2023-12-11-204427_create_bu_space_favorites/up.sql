CREATE TABLE bu_space_favorites (
    id SERIAL PRIMARY KEY,
    bu_space_id integer NOT NULL,
    user_id UUID NOT NULL,
    created_at timestamp NOT NULL DEFAULT (now()),

    -- Unique constraint to ensure a user can't favorite the same space more than once
    CONSTRAINT unique_user_bu_space UNIQUE (user_id, bu_space_id)
);

-- Index for querying all favorite spaces for a specific user_id
CREATE INDEX idx_bu_space_favorites_user_id ON bu_space_favorites(user_id);
