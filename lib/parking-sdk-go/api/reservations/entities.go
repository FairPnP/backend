package api

import "time"

type Reservation struct {
	ID        string    `json:"id"`
	SpaceID   string    `json:"space_id"`
	StartDate time.Time `json:"start_date"`
	EndDate   time.Time `json:"end_date"`
	Status    string    `json:"status"`
}
