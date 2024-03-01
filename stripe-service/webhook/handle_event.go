package webhook

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"

	"github.com/stripe/stripe-go/v76"
)

func HandleEvent(w http.ResponseWriter, event stripe.Event) {
	// Unmarshal the event data into an appropriate struct depending on its Type
	go func(e stripe.Event) {
		switch event.Type {
		case "payment_intent.succeeded":
			var paymentIntent stripe.PaymentIntent
			err := json.Unmarshal(event.Data.Raw, &paymentIntent)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error parsing webhook JSON: %v\n", err)
				// w.WriteHeader(http.StatusBadRequest)
				return
			}

			HandlePaymentIntentSucceeded(paymentIntent)
		case "payment_method.attached":
			var paymentMethod stripe.PaymentMethod
			err := json.Unmarshal(event.Data.Raw, &paymentMethod)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error parsing webhook JSON: %v\n", err)
				// w.WriteHeader(http.StatusBadRequest)
				return
			}

			HandlePaymentMethodAttached(paymentMethod)
		default:
			fmt.Fprintf(os.Stderr, "Unhandled event type: %s\n", event.Type)
		}
	}(event)

	w.WriteHeader(http.StatusOK)
}
