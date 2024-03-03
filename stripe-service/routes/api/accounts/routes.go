package accounts

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.RouterGroup, appState *app.AppState) {
	apiGroup := router.Group("/accounts/v1")
	{
		apiGroup.POST("/dashboard", Dashboard(appState))
		apiGroup.GET("", GetAccount(appState))
	}
}
