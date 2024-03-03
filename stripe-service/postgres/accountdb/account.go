package accountdb

import (
	"time"

	"github.com/google/uuid"
)

type Account struct {
	ID           int       `db:"id"`
	UserID       uuid.UUID `db:"user_id"`
	AccountID    string    `db:"account_id"`
	CreatedAt    time.Time `db:"created_at"`
	LastModified time.Time `db:"last_modified"`
}
