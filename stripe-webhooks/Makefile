.PHONY: stripe-listen stripe-test

stripe-listen:

    stripe listen --forward-to localhost:3000/webhook &

stripe-test:

    stripe trigger payment_intent.succeeded

