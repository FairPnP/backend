package events

import (
	"log"
	"stripe-service/app"

	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentMethodAttached(appState *app.AppState, paymentMethod stripe.PaymentMethod) {
	log.Printf("Payment method %s was attached to a customer.", paymentMethod.ID)
}
