package events

import (
	"log"
	"stripe-service/app"

	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentIntentSucceeded(appState *app.AppState, paymentIntent stripe.PaymentIntent) error {
	log.Printf("Successful payment for %d.", paymentIntent.Amount)

	return nil
}
