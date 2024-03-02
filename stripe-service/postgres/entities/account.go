package entities

import (
	"time"

	"github.com/google/uuid"
)

type Account struct {
	CreatedAt    time.Time `db:"created_at"`
	LastModified time.Time `db:"last_modified"`
	AccountID    string    `db:"account_id"`
	ID           int       `db:"id"`
	UserID       uuid.UUID `db:"user_id"`
}
