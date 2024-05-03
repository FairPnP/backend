package events

import (
	"stripe-webhooks/app"
	"stripe-webhooks/service"

	"github.com/rs/zerolog/log"
	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentIntentSucceeded(appState *app.AppState, paymentIntent stripe.PaymentIntent) error {
	log.Info().Int64("amount", paymentIntent.Amount).Msg("Successful payment for payment intent.")

	userId := paymentIntent.Metadata["user_id"]
	reservationId := paymentIntent.Metadata["reservation_id"]

	service.UpdateReservation(appState.HttpClient, userId, reservationId)

	return nil
}
