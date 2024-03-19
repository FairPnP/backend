package events

import (
	"stripe-webhooks/app"

	"github.com/rs/zerolog/log"
	"github.com/stripe/stripe-go/v76"
)

func HandleAccountUpdated(appState *app.AppState, account stripe.Account) error {
	log.Info().Msgf("Account Updated: %s", account.ID)

	return nil
}
