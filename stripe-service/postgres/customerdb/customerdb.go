package customerdb

import (
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
)

func Insert(pool *sqlx.DB, userID uuid.UUID, customerID string) (*Account, error) {
	var stripeAccount Account
	err := pool.QueryRowx(
		"INSERT INTO stripe_customers (user_id, customer_id) VALUES ($1, $2) RETURNING *",
		userID, customerID,
	).StructScan(&stripeAccount)
	return &stripeAccount, err
}

func Get(pool *sqlx.DB, userID uuid.UUID) (*Account, error) {
	var stripeAccount Account
	err := pool.QueryRowx("SELECT * FROM stripe_customers WHERE user_id = $1", userID).StructScan(&stripeAccount)
	return &stripeAccount, err
}
