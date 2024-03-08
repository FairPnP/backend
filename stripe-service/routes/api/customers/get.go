package customers

import (
	"net/http"
	"stripe-service/app"
	"stripe-service/apperror"
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
			c.Error(err)
			return
		}

		cus, err := customerdb.Get(appState.DB, userId)
		if err != nil {
			apperror.HandleDBError(c, err)
			return
		}

		c.JSON(http.StatusOK, GetCustomerResponse{CustomerId: cus.CustomerID})
	}
}
