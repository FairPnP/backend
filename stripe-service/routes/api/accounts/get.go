package accounts

import (
	"database/sql"
	"net/http"
	"stripe-service/app"
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
			c.Status(http.StatusUnauthorized)
			return
		}

		acc, err := accountdb.Get(appState.DB, userId)
		if err != nil {
			if err == sql.ErrNoRows {
				c.JSON(http.StatusNotFound, gin.H{"error": "Account not found"})
				return
			}

			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		}

		c.JSON(http.StatusOK, GetAccountResponse{AccountId: acc.AccountID})
	}
}
