CREATE TABLE user_listings (
    id SERIAL PRIMARY KEY,
    bu_space_id integer NOT NULL,
    user_id UUID NOT NULL,
    start_date timestamp NOT NULL,
    end_date timestamp NOT NULL,
    price decimal NOT NULL
);

CREATE INDEX idx_user_listings_bu_space_id ON user_listings(bu_space_id);
CREATE INDEX idx_user_listings_user_id ON user_listings(user_id);
CREATE INDEX idx_user_listings_start_date ON user_listings(start_date);
CREATE INDEX idx_user_listings_end_date ON user_listings(end_date);
