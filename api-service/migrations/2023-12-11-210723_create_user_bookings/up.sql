CREATE TABLE user_bookings (
    id SERIAL PRIMARY KEY,
    listing_id integer NOT NULL,
    user_id UUID NOT NULL,
    start_date timestamp NOT NULL,
    end_date timestamp NOT NULL,
    status varchar NOT NULL
);

CREATE INDEX idx_user_bookings_listing_id ON user_bookings(listing_id);
CREATE INDEX idx_user_bookings_user_id ON user_bookings(user_id);
CREATE INDEX idx_user_bookings_start_date ON user_bookings(start_date);
CREATE INDEX idx_user_bookings_end_date ON user_bookings(end_date);
