package utils

import "github.com/gin-gonic/gin"

func StripeMetadata(c *gin.Context) map[string]string {
	requestID, _ := c.Get("requestID")

	// add request_id to metadata
	metadata := map[string]string{
		"request_id": requestID.(string),
	}

	// try get userId
	userID, exists := c.Get("userID")
	if exists {
		metadata["user_id"] = userID.(string)
	}

	return metadata
}
