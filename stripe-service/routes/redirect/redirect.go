package redirect

import (
	"net/http"
	"os"
	"stripe-service/app"

	"github.com/gin-gonic/gin"
)

func SetupRoutes(router *gin.Engine, appState *app.AppState) {
	router.GET("/redirect/stripe/return", func(c *gin.Context) {
		returnUrl := os.Getenv("STRIPE_RETURN_URL")
		c.Redirect(http.StatusFound, returnUrl)
	})

	router.GET("/redirect/stripe/refresh", func(c *gin.Context) {
		returnUrl := os.Getenv("STRIPE_REFRESH_URL")
		c.Redirect(http.StatusFound, returnUrl)
	})
}
