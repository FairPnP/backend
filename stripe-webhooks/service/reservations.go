package service

import (
	api "fairpnp.com/parking-sdk-go/api/reservations"
	"fairpnp.com/parking-sdk-go/client"
	"github.com/rs/zerolog/log"
)

func UpdateReservation(c *client.Client, userId, reservationID string) error {
	status := "confirmed"
	req := api.UpdateReservationRequest{
		Status: &status,
	}

	res, err := api.UpdateReservation(c, userId, reservationID, req)
	if err != nil {
		return err
	}

	log.Info().Any("reservation", res.Reservation).Msg("Reservation updated.")

	return nil
}
