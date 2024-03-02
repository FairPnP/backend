package api

import (
	"stripe-service/app"
	"stripe-service/auth"
	"stripe-service/routes/api/accounts"

	"github.com/gin-gonic/gin"
)

// SetupRoutes configures the API routes
func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	// Example of setting up a route group for /api
	apiGroup := router.Group("/api")
	apiGroup.Use(auth.JWTAuthMiddleware(appState))
	{
		accounts.SetupRoutes(apiGroup, appState)
	}
}
