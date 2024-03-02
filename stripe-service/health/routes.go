// api/health/routes.go
package health

import (
	"context"
	"net/http"
	"stripe-service/app"

	"github.com/gin-gonic/gin"
	"github.com/jmoiron/sqlx"
)

// SetupHealthRoutes registers health-related routes
func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	router.GET("/health", func(c *gin.Context) {
		if err := healthCheck(appState); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Service Unhealthy", "error": err.Error()})
			return
		}
		c.Status(http.StatusOK)
	})
}

// healthCheck performs the actual health checks for your services
func healthCheck(appState *app.AppState) error {
	// Database check
	if err := dbHealthCheck(appState.DB); err != nil {
		return err
	}

	return nil
}

// dbHealthCheck checks the health of the database connection
func dbHealthCheck(db *sqlx.DB) error {
	if err := db.PingContext(context.Background()); err != nil {
		return err
	}
	return nil
}
