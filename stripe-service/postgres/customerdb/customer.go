package customerdb

import (
	"time"

	"github.com/google/uuid"
)

type Account struct {
	ID           int       `db:"id"`
	UserID       uuid.UUID `db:"user_id"`
	CustomerID   string    `db:"customer_id"`
	CreatedAt    time.Time `db:"created_at"`
	LastModified time.Time `db:"last_modified"`
}
