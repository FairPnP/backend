package accounts

import (
	"database/sql"
	"net/http"
	"os"
	"stripe-service/app"
	"stripe-service/apperror"
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
			c.Error(err)
			return
		}

		// Check if account already exists
		accountEntity, err := accountdb.Get(appState.DB, userId)
		if err != nil {
			// If not found, create a new Stripe account
			if err == sql.ErrNoRows {
				params := &stripe.AccountParams{
					Type:    stripe.String(string(stripe.AccountTypeExpress)),
					Country: stripe.String("CA"),
					Capabilities: &stripe.AccountCapabilitiesParams{
						Transfers: &stripe.AccountCapabilitiesTransfersParams{
							Requested: stripe.Bool(true),
						},
					},
					Email:        stripe.String("kylkrie@gmail.com"),
					BusinessType: stripe.String(string(stripe.AccountBusinessTypeIndividual)),
					Individual: &stripe.PersonParams{
						Email:     stripe.String("kylkrie@gmail.com"),
						FirstName: stripe.String("Kyle"),
						LastName:  stripe.String("Smith"),
					},
				}
				acc, err := account.New(params)
				if err != nil {
					apperror.HandleStripeError(c, err)
					return
				}

				// Insert account into database
				accountEntity, err = accountdb.Insert(appState.DB, userId, acc.ID)
				if err != nil {
					apperror.HandleDBError(c, err)
					return
				}
			} else {
				apperror.HandleDBError(c, err)
				return
			}
		}

		// Check if account is already verified
		acc, err := account.GetByID(accountEntity.AccountID, nil)
		if err != nil {
			apperror.HandleStripeError(c, err)
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
				apperror.HandleStripeError(c, err)
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
				apperror.HandleStripeError(c, err)
				return
			}
			link = accLink.URL
		}

		c.JSON(http.StatusOK, DashboardResponse{Link: link})
	}
}
