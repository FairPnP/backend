.PHONY: stripe-listen stripe-test

stripe-listen:
	stripe listen --forward-to localhost:3000/webhooks > stripe_listen.log 2>&1 &

stripe-test:
	stripe trigger payment_intent.succeeded
