DROP TRIGGER IF EXISTS update_space_images_modtime ON space_images;
DROP INDEX IF EXISTS idx_space_images_space_id;
DROP TABLE IF EXISTS space_images;
DROP TYPE IF EXISTS image_status;
