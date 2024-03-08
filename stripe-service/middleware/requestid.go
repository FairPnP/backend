package middleware

import (
	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func RequestID() gin.HandlerFunc {
	return func(c *gin.Context) {
		// Check if the client sent a request ID, otherwise generate one
		requestID := c.GetHeader("X-Request-ID")
		if requestID == "" {
			requestID = uuid.New().String()
			// set the request ID back on the response header
			c.Writer.Header().Set("X-Request-ID", requestID)
		}

		// Set the request ID into the context
		c.Set("requestID", requestID)

		c.Next()
	}
}
