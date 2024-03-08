// ./auth/user.go
package auth

import (
	"net/http"
	"stripe-service/apperror"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
)

func GetUserId(c *gin.Context) (uuid.UUID, error) {
	claims, exists := c.Get("claims")
	if !exists {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Missing claims")
	}

	mapClaims, ok := claims.(jwt.MapClaims)
	if !ok {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Invalid claims type")
	}

	userID, exists := mapClaims["sub"]
	if !exists {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Missing user ID claim")
	}

	userIDStr, ok := userID.(string)
	if !ok {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Invalid user ID claim type")
	}

	uid, err := uuid.Parse(userIDStr)
	if err != nil {
		return uuid.Nil, apperror.New(http.StatusUnauthorized, "Invalid user ID")
	}

	return uid, nil
}
