package auth

import (
	"net/http"
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func AuthMiddleware(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		userId := c.GetHeader("X-Auth-User")
		if userId == "" {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Unauthorized"})
			c.Abort()
			return
		}

		c.Set("userID", userId)

		c.Next()
	}
}
