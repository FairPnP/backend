package app

import (
	"stripe-webhooks/postgres"

	"fairpnp.com/parking-sdk-go/client"
	"github.com/jmoiron/sqlx"
)

type AppState struct {
	DB         *sqlx.DB
	HttpClient *client.Client
}

func CreateAppState(apiUrl string) (*AppState, error) {
	dbpool, err := postgres.CreatePool()
	if err != nil {
		return nil, err
	}

	httpClient := client.NewClient(apiUrl)

	appState := &AppState{
		DB:         dbpool,
		HttpClient: httpClient,
	}

	return appState, nil
}

func (a *AppState) Close() {
	if a.DB != nil {
		a.DB.Close()
	}
}
