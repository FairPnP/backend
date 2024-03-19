package webhook

import (
	"io"
	"net/http"
	"os"
	"stripe-service/app"
	"stripe-service/postgres/eventdb"
	"stripe-service/routes/webhook/events"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"
	"github.com/stripe/stripe-go/v76/webhook"
)

func HandleWebhook(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		const MaxBodyBytes = int64(65536)
		c.Request.Body = http.MaxBytesReader(c.Writer, c.Request.Body, MaxBodyBytes)

		payload, err := io.ReadAll(c.Request.Body)
		if err != nil {
			log.Error().Err(err).Msg("Error reading request body")
			c.AbortWithStatus(http.StatusServiceUnavailable)
			return
		}

		// Your endpoint's unique secret
		endpointSecret := os.Getenv("STRIPE_WEBHOOK_SECRET")

		// Verify the webhook signature
		event, err := webhook.ConstructEvent(payload, c.GetHeader("Stripe-Signature"), endpointSecret)
		if err != nil {
			log.Error().Err(err).Msg("Error verifying webhook signature")
			c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"error": "Invalid signature"})
			return
		}

		// Insert the event into the database
		log.Debug().Str("event_type", string(event.Type)).Msg("Inserting event into database")
		_, err = eventdb.Insert(appState.DB, event.Account, event.ID, string(event.Type), eventdb.StatusReceived)
		if err != nil {
			log.Error().Err(err).Msg("Error inserting event into database")
			c.AbortWithStatus(http.StatusInternalServerError)
			return
		}

		log.Debug().Str("event_type", string(event.Type)).Msg("Handling event")
		// Handle the event
		events.HandleEvent(appState, event)

		c.Status(http.StatusOK)
	}
}
