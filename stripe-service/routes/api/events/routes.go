package events

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.RouterGroup, appState *app.AppState) {
	apiGroup := router.Group("/events/v1")
	{
		apiGroup.GET("", ListEvents(appState))
	}
}
