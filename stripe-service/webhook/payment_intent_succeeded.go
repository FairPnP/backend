package webhook

import (
	"log"

	"github.com/stripe/stripe-go/v76"
)

func HandlePaymentIntentSucceeded(paymentIntent stripe.PaymentIntent) {
	log.Printf("Successful payment for %d.", paymentIntent.Amount)
}
