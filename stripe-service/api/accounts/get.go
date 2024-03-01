package accounts

import (
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func PostDashboard(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		// Implement the logic to get a user by ID
		c.JSON(200, gin.H{
			"message": "user details",
		})
	}
}
