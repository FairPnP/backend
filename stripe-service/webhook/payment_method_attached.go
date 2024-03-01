package webhook

import "github.com/stripe/stripe-go/v76"

func HandlePaymentMethodAttached(paymentMethod stripe.PaymentMethod) {
	println("PaymentMethod attached: " + paymentMethod.ID)
}
