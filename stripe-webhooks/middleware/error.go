package middleware

import (
	"net/http"
	"stripe-webhooks/apperror"

	"github.com/gin-gonic/gin"
)

func ErrorHandler() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Next()

		if len(c.Errors) > 0 {
			for _, err := range c.Errors {
				switch e := err.Err.(type) {
				case *apperror.AppError:
					c.JSON(e.Code, gin.H{"error": e.Message})
				default:
					c.JSON(http.StatusInternalServerError, gin.H{"error": "Internal Server Error"})
				}
			}
			c.Abort()
		}
	}
}
