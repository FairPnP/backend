package accounts

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.RouterGroup, appState *app.AppState) {
	// Example of setting up a route group for /api
	apiGroup := router.Group("/accounts/v1")
	{
		apiGroup.POST("/dashboard", Dashboard(appState))
	}
}
