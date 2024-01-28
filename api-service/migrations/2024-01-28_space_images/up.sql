CREATE TYPE image_status AS ENUM ('pending', 'approved', 'rejected');
CREATE TABLE space_images (
    id SERIAL PRIMARY KEY,
    space_id integer NOT NULL,
    slot_id integer NOT NULL,
    img_url text NOT NULL,
    status image_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW()

    CONSTRAINT unique_space_slot UNIQUE (space_id, slot_id)
);

CREATE INDEX idx_space_images_space_id ON space_images(space_id);

CREATE TRIGGER update_space_images_modtime
BEFORE UPDATE ON space_images
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
