package accountdb

import (
	"stripe-service/postgres/entities"

	"github.com/jmoiron/sqlx"
)

func Insert(pool *sqlx.DB, userID string, accountID string) (*entities.Account, error) {
	var stripeAccount entities.Account
	err := pool.QueryRowx(
		"INSERT INTO stripe_accounts (user_id, account_id) VALUES ($1, $2) RETURNING *",
		userID, accountID,
	).StructScan(&stripeAccount)
	return &stripeAccount, err
}

func Get(pool *sqlx.DB, userID string) (*entities.Account, error) {
	var stripeAccount entities.Account
	err := pool.QueryRowx("SELECT * FROM stripe_accounts WHERE user_id = $1", userID).StructScan(&stripeAccount)
	return &stripeAccount, err
}

func Update(pool *sqlx.DB, userID string, accountID *string) (*entities.Account, error) {
	var stripeAccount entities.Account
	err := pool.QueryRowx(
		"UPDATE stripe_accounts SET account_id = COALESCE($1, account_id) WHERE user_id = $2 RETURNING *",
		accountID, userID,
	).StructScan(&stripeAccount)
	return &stripeAccount, err
}

func Delete(pool *sqlx.DB, userID string) error {
	_, err := pool.Exec("DELETE FROM stripe_accounts WHERE user_id = $1", userID)
	return err
}
