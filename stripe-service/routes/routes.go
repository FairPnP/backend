package routes

import (
	"stripe-service/app"
	"stripe-service/middleware"
	"stripe-service/routes/api"
	"stripe-service/routes/health"
	"stripe-service/routes/redirect"
	"stripe-service/routes/webhook"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	router.Use(middleware.Logger())
	router.Use(middleware.RequestID())
	router.Use(middleware.ErrorHandler())

	health.SetupRoutes(router, appState)
	api.SetupRoutes(router, appState)
	redirect.SetupRoutes(router, appState)

	router.POST("/webhook", webhook.HandleWebhook(appState))
}
