package events

import (
	"log"
	"stripe-service/app"

	"github.com/stripe/stripe-go/v76"
)

func HandleAccountUpdated(appState *app.AppState, account stripe.Account) {
	log.Printf("Account updated: %v\n", account.ID)
}
