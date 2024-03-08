package accounts

import (
	"net/http"
	"stripe-service/app"
	"stripe-service/apperror"
	"stripe-service/auth"
	"stripe-service/postgres/accountdb"

	"github.com/gin-gonic/gin"
)

type GetAccountResponse struct {
	AccountId string `json:"account_id"`
}

func GetAccount(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		userId, err := auth.GetUserId(c)
		if err != nil {
			c.Error(err)
			return
		}

		acc, err := accountdb.Get(appState.DB, userId)
		if err != nil {
			apperror.HandleDBError(c, err)
			return
		}

		c.JSON(http.StatusOK, GetAccountResponse{AccountId: acc.AccountID})
	}
}
