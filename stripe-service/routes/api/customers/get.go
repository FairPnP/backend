package customers

import (
	"database/sql"
	"net/http"
	"stripe-service/app"
	"stripe-service/auth"
	"stripe-service/postgres/customerdb"

	"github.com/gin-gonic/gin"
)

type GetCustomerResponse struct {
	CustomerId string `json:"customer_id"`
}

func GetCustomer(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		userId, err := auth.GetUserId(c)
		if err != nil {
			c.Status(http.StatusUnauthorized)
			return
		}

		cus, err := customerdb.Get(appState.DB, userId)
		if err != nil {
			if err == sql.ErrNoRows {
				c.JSON(http.StatusNotFound, gin.H{"error": "Customer not found"})
				return
			}

			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		}

		c.JSON(http.StatusOK, GetCustomerResponse{CustomerId: cus.CustomerID})
	}
}
