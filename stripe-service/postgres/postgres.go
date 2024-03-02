package postgres

import (
	"fmt"
	"log"
	"os"

	_ "github.com/jackc/pgx/v5/stdlib"
	"github.com/jmoiron/sqlx"
)

func CreatePool() (*sqlx.DB, error) {
	databaseURL := os.Getenv("DATABASE_URL")
	if databaseURL == "" {
		return nil, fmt.Errorf("DATABASE_URL environment variable not set")
	}

	db, err := sqlx.Open("pgx", databaseURL)
	if err != nil {
		log.Printf("Unable to create connection pool: %v\n", err)
		return nil, err
	}

	db.SetMaxOpenConns(8)

	return db, nil
}
