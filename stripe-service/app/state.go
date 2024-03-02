package app

import (
	"log"
	"os"
	"stripe-service/postgres"

	"github.com/MicahParks/keyfunc/v3"
	"github.com/jmoiron/sqlx"
)

type AppState struct {
	DB         *sqlx.DB
	JwtKeyFunc keyfunc.Keyfunc
}

func CreateAppState() (*AppState, error) {
	jwksURL := os.Getenv("AUTH_JWKS_URL")
	jwks, err := keyfunc.NewDefault([]string{jwksURL})
	if err != nil {
		log.Printf("Failed to create JWK Set from resource at the given URL.\nError: %s", err)
		return nil, err
	}

	dbpool, err := postgres.CreatePool()
	if err != nil {
		log.Printf("Failed to create pool: %v\n", err)
		return nil, err
	}

	appState := &AppState{
		DB:         dbpool,
		JwtKeyFunc: jwks,
	}

	return appState, nil
}

func (a *AppState) Close() {
	if a.DB != nil {
		a.DB.Close()
	}
}
