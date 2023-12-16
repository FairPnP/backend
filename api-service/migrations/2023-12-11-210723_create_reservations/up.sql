CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    availability_id integer NOT NULL,
    start_date timestamp NOT NULL,
    end_date timestamp NOT NULL CHECK (end_date > start_date),
    created_at timestamp NOT NULL DEFAULT NOW(),
    last_modified timestamp NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reservations_availability_id ON reservations(availability_id);
CREATE INDEX idx_reservations_user_id ON reservations(user_id);
CREATE INDEX idx_reservations_start_date ON reservations(start_date);
CREATE INDEX idx_reservations_end_date ON reservations(end_date);

CREATE TRIGGER update_reservations_modtime
BEFORE UPDATE ON reservations
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
