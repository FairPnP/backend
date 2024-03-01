package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"stripe-service/webhook"

	"github.com/stripe/stripe-go/v76"
)

func main() {
	stripe.Key = os.Getenv("STRIPE_SECRET_KEY")

	http.HandleFunc("/webhook", webhook.HandleWebhook)
	port := os.Getenv("WEB_SERVER_PORT")
	addr := fmt.Sprintf(":%s", port)
	log.Printf("Listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
