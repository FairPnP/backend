package events

import (
	"stripe-service/app"
	"stripe-service/postgres/accountdb"

	"github.com/stripe/stripe-go/v76"
)

func HandleAccountUpdated(appState *app.AppState, account stripe.Account) error {
	accountdb.Update(appState.DB, account.ID, account.DetailsSubmitted, string(account.Capabilities.Transfers))

	return nil
}
