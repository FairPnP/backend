CREATE TABLE stripe_customers (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    customer_id VARCHAR(255) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    last_modified timestamp NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_user_customer_id UNIQUE (user_id, customer_id)
);

CREATE INDEX idx_stripe_customers_customer_id ON stripe_customers(customer_id);
CREATE INDEX idx_stripe_customers_user_id ON stripe_customers(user_id);

CREATE TRIGGER update_stripe_customers_modtime
BEFORE UPDATE ON stripe_customers
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
