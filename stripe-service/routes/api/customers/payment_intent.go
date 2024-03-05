package customers

import (
	"database/sql"
	"log"
	"net/http"
	"stripe-service/app"
	"stripe-service/auth"
	"stripe-service/postgres/customerdb"

	"github.com/gin-gonic/gin"
	gonanoid "github.com/matoous/go-nanoid/v2"
	"github.com/stripe/stripe-go/v76"
	"github.com/stripe/stripe-go/v76/customer"
	"github.com/stripe/stripe-go/v76/ephemeralkey"
	"github.com/stripe/stripe-go/v76/paymentintent"
)

type PaymentIntentRequest struct {
	Amount int64 `json:"amount"`
}

type PaymentIntentResponse struct {
	CustomerId   string `json:"customer_id"`
	ClientSecret string `json:"client_secret"`
	EphemeralKey string `json:"ephemeral_key"`
}

func PostIntent(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		userId, err := auth.GetUserId(c)
		if err != nil {
			c.Status(http.StatusUnauthorized)
			return
		}

		// parse body data
		var req PaymentIntentRequest
		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(400, gin.H{"error": "Invalid request body"})
			return
		}

		// get customer id
		cus, err := customerdb.Get(appState.DB, userId)
		if err != nil {
			if err == sql.ErrNoRows {
				// create stripe customer
				stripeCustomer, err := customer.New(
					&stripe.CustomerParams{
						Description: stripe.String("Customer for user " + userId.String()),
					},
				)
				if err != nil {
					c.JSON(500, gin.H{"error": "Failed to create customer"})
					return
				}

				// insert customer into database
				cus, err = customerdb.Insert(appState.DB, userId, stripeCustomer.ID)
				if err != nil {
					c.JSON(500, gin.H{"error": "Failed to insert customer into database"})
					return
				}

			}
		}

		// generate unique transfer group id
		transferGroup, err := gonanoid.New()
		if err != nil {
			c.JSON(500, gin.H{"error": "Failed to generate transfer group id"})
			return
		}
		transferGroup = "tg_" + transferGroup

		// create ephemeral key
		customerID := cus.CustomerID
		ephemeralKey, err := ephemeralkey.New(
			&stripe.EphemeralKeyParams{
				Customer:      stripe.String(customerID),
				StripeVersion: stripe.String("2023-10-16"),
			},
		)
		if err != nil {
			log.Println(err)
			c.JSON(500, gin.H{"error": "Failed to create ephemeral key"})
			return
		}

		// create payment intent
		result, err := paymentintent.New(
			&stripe.PaymentIntentParams{
				Amount:        stripe.Int64(req.Amount),
				Currency:      stripe.String(string(stripe.CurrencyCAD)),
				TransferGroup: stripe.String(transferGroup),
			},
		)
		if err != nil {
			c.JSON(500, gin.H{"error": "Failed to create payment intent"})
			return
		}

		// return response
		c.JSON(200, PaymentIntentResponse{
			CustomerId:   customerID,
			ClientSecret: result.ClientSecret,
			EphemeralKey: ephemeralKey.Secret,
		})
	}
}
