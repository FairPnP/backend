package auth

import (
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

func GetUserId(c *gin.Context) string {
	// get claims from context
	claims := c.MustGet("claims").(jwt.MapClaims)

	// get user_id from claims
	user_id := claims["sub"].(string)

	return user_id
}
