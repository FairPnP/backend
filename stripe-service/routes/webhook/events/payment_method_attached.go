package events

import (
	"stripe-service/app"

	"github.com/rs/zerolog/log"
	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentMethodAttached(appState *app.AppState, paymentMethod stripe.PaymentMethod) error {
	log.Info().Str("payment_method_id", paymentMethod.ID).Msg("Payment method was attached to a customer.")

	return nil
}
