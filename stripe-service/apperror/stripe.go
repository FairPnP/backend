package apperror

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/stripe/stripe-go/v76"
)

func HandleStripeError(c *gin.Context, err error) {
	stripeErr, ok := err.(*stripe.Error)
	if !ok {
		c.Error(New(http.StatusInternalServerError, "Stripe API error"))
		return
	}

	switch stripeErr.Code {
	case stripe.ErrorCodeResourceMissing:
		c.Error(New(http.StatusNotFound, "Stripe resource not found"))
	case stripe.ErrorCodeAuthenticationRequired:
		c.Error(New(http.StatusUnauthorized, "Stripe authentication required"))
	default:
		c.Error(New(http.StatusInternalServerError, "Stripe API error"))
	}
}
