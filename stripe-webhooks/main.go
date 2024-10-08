package main

import (
	"context"
	"net/http"
	"os"
	"os/signal"
	"stripe-webhooks/app"
	"stripe-webhooks/routes"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	zerolog.SetGlobalLevel(zerolog.InfoLevel)
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: time.RFC3339})

	log.Info().Msg("Starting server...")

	apiUrl := os.Getenv("API_URL")
	appState, err := app.CreateAppState(apiUrl)
	if err != nil {
		log.Fatal().Err(err).Msg("Error creating app state")
	}

	router := gin.New()
	routes.SetupRoutes(router, appState)

	port := os.Getenv("WEB_SERVER_PORT")
	server := &http.Server{
		Addr:    ":" + port,
		Handler: router,
	}

	// Start the server
	go func() {
		log.Info().Msgf("Starting server on port %s", port)
		if err := server.ListenAndServe(); err != http.ErrServerClosed {
			log.Fatal().Err(err).Msg("HTTP server ListenAndServe")
		}
	}()

	// Setup channel to listen for termination signals
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	// Block until a signal is received.
	<-c
	log.Info().Msg("Shutting down gracefully...")

	// Create a deadline to wait for.
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Doesn't block if no connections, but will wait until the timeout deadline.
	if err := server.Shutdown(ctx); err != nil {
		log.Fatal().Err(err).Msg("HTTP server Shutdown")
	}

	// Now that the server has shut down, we can close other resources.
	appState.Close()
}
