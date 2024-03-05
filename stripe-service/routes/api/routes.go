package api

import (
	"stripe-service/app"
	"stripe-service/auth"
	"stripe-service/routes/api/accounts"
	"stripe-service/routes/api/customers"
	"stripe-service/routes/api/events"

	"github.com/gin-gonic/gin"
)

// SetupRoutes configures the API routes
func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	apiGroup := router.Group("/api")
	apiGroup.Use(auth.JWTAuthMiddleware(appState))
	{
		accounts.SetupRoutes(apiGroup, appState)
		customers.SetupRoutes(apiGroup, appState)
		events.SetupRoutes(apiGroup, appState)
	}
}
