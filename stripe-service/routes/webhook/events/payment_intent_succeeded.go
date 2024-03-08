package events

import (
	"stripe-service/app"

	"github.com/rs/zerolog/log"
	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentIntentSucceeded(appState *app.AppState, paymentIntent stripe.PaymentIntent) error {
	log.Info().Int64("amount", paymentIntent.Amount).Msg("Successful payment for payment intent.")

	return nil
}
