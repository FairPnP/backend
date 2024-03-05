CREATE TABLE stripe_events (
    id SERIAL PRIMARY KEY,
    account_id VARCHAR(255) NOT NULL,
    event_id VARCHAR(255) NOT NULL UNIQUE,
    event_type VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_status CHECK (status IN ('received', 'processing', 'processed', 'failed'))
);

CREATE INDEX idx_stripe_events_account_id ON stripe_events(account_id);
CREATE INDEX idx_stripe_events_event_type ON stripe_events(event_type);
CREATE INDEX idx_stripe_events_status ON stripe_events(status);

CREATE TRIGGER update_stripe_events_modtime
BEFORE UPDATE ON stripe_events
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
