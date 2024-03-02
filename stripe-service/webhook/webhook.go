package webhook

import (
	"io"
	"log"
	"net/http"
	"os"
	"stripe-service/app"
	"stripe-service/webhook/events"

	"github.com/gin-gonic/gin"
	"github.com/stripe/stripe-go/v76/webhook"
)

func HandleWebhook(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		const MaxBodyBytes = int64(65536)
		c.Request.Body = http.MaxBytesReader(c.Writer, c.Request.Body, MaxBodyBytes)

		payload, err := io.ReadAll(c.Request.Body)
		if err != nil {
			log.Printf("Error reading request body: %v\n", err)
			c.AbortWithStatus(http.StatusServiceUnavailable)
			return
		}

		// Your endpoint's unique secret
		endpointSecret := os.Getenv("STRIPE_WEBHOOK_SECRET")

		// Verify the webhook signature
		event, err := webhook.ConstructEvent(payload, c.GetHeader("Stripe-Signature"), endpointSecret)
		if err != nil {
			log.Printf("Error verifying webhook signature: %v\n", err)
			c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"error": "Invalid signature"})
			return
		}

		// Handle the event
		events.HandleEvent(appState, event)

		c.Status(http.StatusOK)
	}
}
