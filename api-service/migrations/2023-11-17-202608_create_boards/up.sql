CREATE TABLE boards (
    id SERIAL PRIMARY KEY,
    team_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    CONSTRAINT unique_name_within_team UNIQUE (team_id, name)
);

CREATE INDEX idx_team_id ON boards (team_id);
