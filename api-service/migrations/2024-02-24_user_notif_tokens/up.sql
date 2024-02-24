CREATE TABLE user_notif_tokens (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    expo_token VARCHAR(255),
    device_token VARCHAR(255) NOT NULL,
    device_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE(user_id, expo_token, device_token)
);

CREATE INDEX idx_user_notif_tokens_user_id ON user_notif_tokens(user_id);

CREATE TRIGGER update_user_notif_tokens_modtime
BEFORE UPDATE ON user_notif_tokens
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();

