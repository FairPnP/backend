package accounts

import (
	"database/sql"
	"net/http"
	"os"
	"stripe-service/app"
	"stripe-service/auth"
	"stripe-service/postgres/accountdb"

	"github.com/gin-gonic/gin"

	"github.com/stripe/stripe-go/v76"
	"github.com/stripe/stripe-go/v76/account"
	"github.com/stripe/stripe-go/v76/accountlink"
	"github.com/stripe/stripe-go/v76/loginlink"
)

type DashboardResponse struct {
	Link string `json:"link"`
}

func Dashboard(appState *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		userId, err := auth.GetUserId(c)
		if err != nil {
			c.Status(http.StatusUnauthorized)
			return
		}

		// Check if account already exists
		accountEntity, err := accountdb.Get(appState.DB, userId)
		if err != nil {
			// If not found, create a new Stripe account
			if err == sql.ErrNoRows {
				params := &stripe.AccountParams{
					Type: stripe.String(string(stripe.AccountTypeExpress)),
				}
				acc, err := account.New(params)
				if err != nil {
					c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
					return
				}

				// Insert account into database
				accountEntity, err = accountdb.Insert(appState.DB, userId, acc.ID)
				if err != nil {
					c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
					return
				}
			} else {
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
		}

		// Check if account is already verified
		acc, err := account.GetByID(accountEntity.AccountID, nil)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		var link string
		if acc.DetailsSubmitted {
			// Create login link if details are already submitted
			loginLinkParams := &stripe.LoginLinkParams{
				Account: stripe.String(accountEntity.AccountID),
			}
			loginLink, err := loginlink.New(loginLinkParams)
			if err != nil {
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			link = loginLink.URL
		} else {
			// Create account link for onboarding
			baseUrl := os.Getenv("BASE_URL")
			refreshUrl := baseUrl + "/redirect/stripe/return"
			returnUrl := baseUrl + "/redirect/stripe/return"
			accountLinkParams := &stripe.AccountLinkParams{
				Account:    stripe.String(accountEntity.AccountID),
				RefreshURL: stripe.String(refreshUrl),
				ReturnURL:  stripe.String(returnUrl),
				Type:       stripe.String("account_onboarding"),
			}
			accLink, err := accountlink.New(accountLinkParams)
			if err != nil {
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			link = accLink.URL
		}

		c.JSON(http.StatusOK, DashboardResponse{Link: link})
	}
}
