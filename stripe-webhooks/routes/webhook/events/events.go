package events

import (
	"encoding/json"
	"stripe-webhooks/app"
	"stripe-webhooks/postgres/eventdb"
	"sync"

	"github.com/rs/zerolog/log"
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
		err := processEvent(appState, task.Event)
		if err != nil {
			log.Error().Err(err).Msg("Error processing event")
		}
	}
}

// processEvent processes the Stripe event based on its type.
func processEvent(appState *app.AppState, event stripe.Event) error {
	// track whether the event was processed successfully
	success := false

	log.Debug().Str("event_type", string(event.Type)).Msg("Processing event 1")

	// update status based on the result of the processing
	defer func() {
		log.Debug().Str("event_type", string(event.Type)).Bool("success", success).Msg("Processing event")
		if success {
			// set status to processed
			err := eventdb.UpdateStatus(appState.DB, event.ID, eventdb.StatusProcessed)
			if err != nil {
				log.Error().Err(err).Msg("Error setting event as processed")
			}
		} else {
			// set status to failed
			err := eventdb.UpdateStatus(appState.DB, event.ID, eventdb.StatusFailed)
			if err != nil {
				log.Error().Err(err).Msg("Error setting event as failed")
			}
		}
	}()

	// set status to processing
	err := eventdb.UpdateStatus(appState.DB, event.ID, eventdb.StatusProcessing)
	if err != nil {
		log.Error().Err(err).Msg("Error setting event as processing")
		return err
	}

	log.Debug().Str("event_type", string(event.Type)).Msg("Processing event 2")

	switch event.Type {
	case "account.updated":
		var account stripe.Account
		if err := json.Unmarshal(event.Data.Raw, &account); err != nil {
			return err
		}
		if err := HandleAccountUpdated(appState, account); err != nil {
			return err
		}
		success = true

	case "payment_intent.succeeded":
		var paymentIntent stripe.PaymentIntent
		if err := json.Unmarshal(event.Data.Raw, &paymentIntent); err != nil {
			return err
		}
		if err := HandlePaymentIntentSucceeded(appState, paymentIntent); err != nil {
			return err
		}
		success = true

	case "payment_method.attached":
		var paymentMethod stripe.PaymentMethod
		if err := json.Unmarshal(event.Data.Raw, &paymentMethod); err != nil {
			return err
		}
		if err := HandlePaymentMethodAttached(appState, paymentMethod); err != nil {
			return err
		}
		success = true

	default:
		log.Warn().Str("event_type", string(event.Type)).Msg("Worker unhandled event type")
	}

	return nil
}

// HandleEvent queues the event to be processed by a worker.
func HandleEvent(appState *app.AppState, event stripe.Event) {
	once.Do(func() {
		// Initialize the worker pool with 6 workers
		initializeWorkerPool(6, appState)
	})

	taskQueue <- Task{Event: event}
}
