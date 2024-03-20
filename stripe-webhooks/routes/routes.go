package routes

import (
	"stripe-webhooks/app"
	"stripe-webhooks/middleware"
	"stripe-webhooks/routes/health"
	"stripe-webhooks/routes/webhook"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	router.Use(middleware.Logger())
	router.Use(middleware.RequestID())
	router.Use(middleware.ErrorHandler())

	health.SetupRoutes(router, appState)

	router.POST("/webhooks", webhook.HandleWebhook(appState))
}
