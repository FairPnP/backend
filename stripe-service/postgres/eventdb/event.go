package eventdb

import (
	"time"
)

type EventStatus string

const (
	StatusReceived   EventStatus = "received"
	StatusProcessing EventStatus = "processing"
	StatusProcessed  EventStatus = "processed"
	StatusFailed     EventStatus = "failed"
)

type StripeEvent struct {
	ID           int         `db:"id" json:"id"`
	AccountID    string      `db:"account_id" json:"account_id"`
	EventID      string      `db:"event_id" json:"event_id"`
	EventType    string      `db:"event_type" json:"event_type"`
	Status       EventStatus `db:"status" json:"status"`
	CreatedAt    time.Time   `db:"created_at" json:"created_at"`
	LastModified time.Time   `db:"last_modified" json:"last_modified"`
}
