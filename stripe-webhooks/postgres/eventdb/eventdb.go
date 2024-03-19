package eventdb

import (
	"github.com/jmoiron/sqlx"
)

func Insert(db *sqlx.DB, accountID, eventID, eventType string, status EventStatus) (*StripeEvent, error) {
	var event StripeEvent
	err := db.QueryRowx(
		"INSERT INTO stripe_events (account_id, event_id, event_type, status) VALUES ($1, $2, $3, $4) RETURNING *",
		accountID, eventID, eventType, status,
	).StructScan(&event)
	if err != nil {
		return nil, err
	}
	return &event, nil
}

func Get(db *sqlx.DB, eventID string) (*StripeEvent, error) {
	var event StripeEvent
	err := db.QueryRowx("SELECT * FROM stripe_events WHERE event_id = $1", eventID).StructScan(&event)
	if err != nil {
		return nil, err
	}
	return &event, nil
}

func UpdateStatus(db *sqlx.DB, eventID string, status EventStatus) error {
	_, err := db.Exec("UPDATE stripe_events SET status = $1 WHERE event_id = $2",
		status, eventID)
	return err
}

func List(db *sqlx.DB) ([]StripeEvent, error) {
	var events []StripeEvent
	err := db.Select(&events, "SELECT * FROM stripe_events")
	if err != nil {
		return nil, err
	}
	return events, nil
}

func ListByAccountId(db *sqlx.DB, accountID string) ([]StripeEvent, error) {
	var events []StripeEvent
	err := db.Select(&events, "SELECT * FROM stripe_events WHERE account_id = $1", accountID)
	if err != nil {
		return nil, err
	}
	return events, nil
}
