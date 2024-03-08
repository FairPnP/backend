// ./auth/user.go
package auth

import (
	"net/http"
	"stripe-service/apperror"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func GetUserId(c *gin.Context) (uuid.UUID, error) {
	userID, exists := c.Get("userID")
	if !exists {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Missing user ID")
	}

	userIDStr, ok := userID.(string)
	if !ok {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Invalid user ID")
	}

	uid, err := uuid.Parse(userIDStr)
	if err != nil {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Invalid user ID")
	}

	return uid, nil
}
