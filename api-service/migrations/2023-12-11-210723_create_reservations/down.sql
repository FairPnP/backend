DROP INDEX IF EXISTS idx_reservations_space_id;
DROP INDEX IF EXISTS idx_reservations_availability_id;
DROP INDEX IF EXISTS idx_reservations_user_id;
DROP INDEX IF EXISTS idx_reservations_start_date;
DROP INDEX IF EXISTS idx_reservations_end_date;
DROP INDEX IF EXISTS idx_reservations_status;
DROP TABLE IF EXISTS reservations;
DROP TYPE IF EXISTS reservation_status;
