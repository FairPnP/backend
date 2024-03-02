package events

import (
	"encoding/json"
	"log"
	"stripe-service/app"
	"sync"

	"github.com/stripe/stripe-go/v76"
)

type Task struct {
	Event stripe.Event
}

var (
	taskQueue chan Task
	once      sync.Once
)

// initializeWorkerPool initializes the worker pool with a given number of workers.
func initializeWorkerPool(numWorkers int, appState *app.AppState) {
	taskQueue = make(chan Task, numWorkers*2) // Buffer can be adjusted based on expected load

	for i := 0; i < numWorkers; i++ {
		go worker(appState)
	}
}

// worker is the function that runs for each worker in the pool, processing tasks.
func worker(appState *app.AppState) {
	for task := range taskQueue {
		processEvent(appState, task.Event)
	}
}

// processEvent processes the Stripe event based on its type.
func processEvent(appState *app.AppState, event stripe.Event) {
	switch event.Type {
	case "payment_intent.succeeded":
		var paymentIntent stripe.PaymentIntent
		if err := json.Unmarshal(event.Data.Raw, &paymentIntent); err != nil {
			log.Printf("Worker error parsing payment_intent.succeeded: %v\n", err)
			return
		}
		HandlePaymentIntentSucceeded(appState, paymentIntent)

	case "payment_method.attached":
		var paymentMethod stripe.PaymentMethod
		if err := json.Unmarshal(event.Data.Raw, &paymentMethod); err != nil {
			log.Printf("Worker error parsing payment_method.attached: %v\n", err)
			return
		}
		HandlePaymentMethodAttached(appState, paymentMethod)

	default:
		log.Printf("Worker unhandled event type: %s\n", event.Type)
	}
}

// HandleEvent queues the event to be processed by a worker.
func HandleEvent(appState *app.AppState, event stripe.Event) {
	once.Do(func() {
		// Initialize the worker pool with 6 workers
		initializeWorkerPool(6, appState)
	})

	taskQueue <- Task{Event: event}
}
