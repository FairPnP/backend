package accounts

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.RouterGroup, appState *app.AppState) {
	apiGroup := router.Group("/accounts/v1")
	{
		apiGroup.GET("", GetAccount(appState))
		apiGroup.POST("/dashboard", Dashboard(appState))
	}
}
