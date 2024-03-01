package events

import (
	"encoding/json"
	"log"
	"stripe-service/app"

	"github.com/stripe/stripe-go/v76"
)

func HandleEvent(appState *app.AppState, event stripe.Event) {
	go func(e stripe.Event) {
		switch event.Type {
		case "payment_intent.succeeded":
			var paymentIntent stripe.PaymentIntent
			err := json.Unmarshal(event.Data.Raw, &paymentIntent)
			if err != nil {
				log.Printf("Error parsing webhook JSON: %v\n", err)
				return
			}

			HandlePaymentIntentSucceeded(appState, paymentIntent)
		case "payment_method.attached":
			var paymentMethod stripe.PaymentMethod
			err := json.Unmarshal(event.Data.Raw, &paymentMethod)
			if err != nil {
				log.Printf("Error parsing webhook JSON: %v\n", err)
				return
			}

			HandlePaymentMethodAttached(appState, paymentMethod)
		default:
			log.Printf("Unhandled event type: %s\n", event.Type)
		}
	}(event)
}
