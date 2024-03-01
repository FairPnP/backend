package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"stripe-service/app"
	"stripe-service/webhook"
	"syscall"
	"time"

	"github.com/stripe/stripe-go/v76"
)

func main() {
	stripe.Key = os.Getenv("STRIPE_SECRET_KEY")

	appState, err := app.CreateAppState()
	if err != nil {
		log.Fatalf("Error creating application state: %v", err)
	}

	http.HandleFunc("/webhook", func(w http.ResponseWriter, req *http.Request) {
		webhook.HandleWebhook(appState, w, req)
	})

	port := os.Getenv("WEB_SERVER_PORT")
	server := &http.Server{Addr: ":" + port}

	// Start the server
	go func() {
		log.Printf("Listening on %s", server.Addr)
		if err := server.ListenAndServe(); err != http.ErrServerClosed {
			log.Fatalf("HTTP server ListenAndServe: %v", err)
		}
	}()

	// Setup channel to listen for termination signals
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	// Block until a signal is received.
	<-c
	log.Println("Shutting down gracefully...")

	// Create a deadline to wait for.
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Doesn't block if no connections, but will wait until the timeout deadline.
	if err := server.Shutdown(ctx); err != nil {
		log.Fatalf("HTTP server Shutdown: %v", err)
	}

	// Now that the server has shut down, we can close other resources.
	appState.Close()
}
