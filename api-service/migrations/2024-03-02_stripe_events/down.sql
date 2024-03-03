DROP TRIGGER IF EXISTS update_stripe_events_modtime ON stripe_events;
DROP INDEX IF EXISTS idx_stripe_events_event_account_id;
DROP INDEX IF EXISTS idx_stripe_events_event_type;
DROP INDEX IF EXISTS idx_stripe_events_status;
DROP TABLE IF EXISTS stripe_events;
