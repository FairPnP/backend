package auth

import (
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
)

func GetUserId(c *gin.Context) (uuid.UUID, error) {
	// get claims from context
	claims := c.MustGet("claims").(jwt.MapClaims)

	// get user_id from claims
	user_id := claims["sub"].(string)

	return uuid.Parse(user_id)
}
