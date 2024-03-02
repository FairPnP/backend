package auth

import (
	"log"
	"net/http"
	"os"
	"strings"
	"stripe-service/app"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

// JWTAuthMiddleware creates a middleware for JWT authentication.
func JWTAuthMiddleware(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		authHeader := c.GetHeader("Authorization")
		if authHeader == "" {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Authorization header is required"})
			c.Abort()
			return
		}

		// Extract the token from the Authorization header.
		tokenString := strings.TrimPrefix(authHeader, "Bearer ")
		if tokenString == authHeader {
			// The prefix was not present
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Bearer token not found"})
			c.Abort()
			return
		}

		// Parse and validate the token.
		// TODO: remove jwt.WithoutClaimsValidation() and add claims validation
		token, err := jwt.Parse(tokenString, appState.JwtKeyFunc.Keyfunc, jwt.WithIssuer(os.Getenv("AUTH_ISSUER")), jwt.WithoutClaimsValidation())
		if err != nil {
			log.Printf("Error parsing token: %v\n", err)
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid or expired token"})
			c.Abort()
			return
		}

		// Ensure the token is valid.
		if !token.Valid {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid token"})
			c.Abort()
			return
		}

		// Token is valid. Attach claims to the context.
		if claims, ok := token.Claims.(jwt.MapClaims); ok && token.Valid {
			c.Set("claims", claims)
		} else {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid token claims"})
			c.Abort()
			return
		}

		c.Next()
	}
}
