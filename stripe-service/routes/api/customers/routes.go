package customers

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.RouterGroup, appState *app.AppState) {
	apiGroup := router.Group("/customers/v1")
	{
		apiGroup.GET("", GetCustomer(appState))
		apiGroup.POST("/payment_intent", PostIntent(appState))
	}
}
