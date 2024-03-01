package app

import (
	"log"
	"stripe-service/postgres"

	"github.com/jackc/pgx/v5/pgxpool"
)

type AppState struct {
	DBPool *pgxpool.Pool
}

func CreateAppState() (*AppState, error) {
	dbpool, err := postgres.CreatePool()
	if err != nil {
		log.Printf("Failed to create pool: %v\n", err)
		return nil, err
	}

	appState := &AppState{
		DBPool: dbpool,
	}

	return appState, nil
}

func (a *AppState) Close() {
	if a.DBPool != nil {
		a.DBPool.Close()
	}
}
