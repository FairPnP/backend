CREATE TABLE availability (
    id SERIAL PRIMARY KEY,
    space_id integer NOT NULL,
    user_id UUID NOT NULL,
    start_date timestamp NOT NULL,
    end_date timestamp NOT NULL CHECK (end_date > start_date),
    hourly_rate decimal NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    last_modified timestamp NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_availability_space_id ON availability(space_id);
CREATE INDEX idx_availability_user_id ON availability(user_id);
CREATE INDEX idx_availability_start_date ON availability(start_date);
CREATE INDEX idx_availability_end_date ON availability(end_date);
CREATE INDEX idx_availability_space_id_dates ON availability(space_id, start_date, end_date);
CREATE INDEX idx_availability_user_id_dates ON availability(user_id, start_date, end_date);

CREATE TRIGGER update_availability_modtime
BEFORE UPDATE ON availability
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
