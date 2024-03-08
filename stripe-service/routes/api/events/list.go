package events

import (
	"net/http"
	"stripe-service/app"
	"stripe-service/apperror"
	"stripe-service/postgres/eventdb"

	"github.com/gin-gonic/gin"
)

type ListEventsResponse struct {
	Events []eventdb.StripeEvent `json:"events"`
}

func ListEvents(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		events, err := eventdb.List(appState.DB)
		if err != nil {
			apperror.HandleDBError(c, err)
			return
		}

		if len(events) == 0 {
			events = []eventdb.StripeEvent{}
		}

		c.JSON(http.StatusOK, ListEventsResponse{Events: events})
	}
}
