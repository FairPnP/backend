package app

import (
	"stripe-webhooks/postgres"

	"github.com/jmoiron/sqlx"
)

type AppState struct {
	DB *sqlx.DB
}

func CreateAppState() (*AppState, error) {
	dbpool, err := postgres.CreatePool()
	if err != nil {
		return nil, err
	}

	appState := &AppState{
		DB: dbpool,
	}

	return appState, nil
}

func (a *AppState) Close() {
	if a.DB != nil {
		a.DB.Close()
	}
}
