package api

import (
	"fmt"
	"time"

	"fairpnp.com/parking-sdk-go/client"
)

type UpdateReservationRequest struct {
	StartDate *time.Time `json:"start_date,omitempty"`
	EndDate   *time.Time `json:"end_date,omitempty"`
	Status    *string    `json:"status,omitempty"`
}

type UpdateReservationResponse struct {
	Reservation Reservation `json:"reservation"`
}

func UpdateReservation(c *client.Client, userId, reservationID string, req UpdateReservationRequest) (*UpdateReservationResponse, error) {
	path := fmt.Sprintf("/reservations/v1/%s", reservationID)

	var res UpdateReservationResponse
	err := c.Put(path, userId, req, &res)
	if err != nil {
		return nil, err
	}

	return &res, nil
}
