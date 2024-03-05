CREATE TABLE stripe_accounts (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    account_id VARCHAR(255) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    last_modified timestamp NOT NULL DEFAULT NOW(),
    details_submitted BOOLEAN NOT NULL DEFAULT FALSE,
    transfers_status VARCHAR(50) NOT NULL DEFAULT 'disabled',

    CONSTRAINT unique_user_account_id UNIQUE (user_id, account_id)
);

CREATE INDEX idx_stripe_accounts_account_id ON stripe_accounts(account_id);
CREATE INDEX idx_stripe_accounts_user_id ON stripe_accounts(user_id);

CREATE TRIGGER update_stripe_accounts_modtime
BEFORE UPDATE ON stripe_accounts
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
