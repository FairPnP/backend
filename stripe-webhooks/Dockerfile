# Start from the official Golang image for a specific version
FROM golang:1.22 as local

WORKDIR /app/stripe-webhooks

# Install air for hot reloading
RUN go install github.com/cosmtrek/air@latest


# Copy go.mod and go.sum to download the dependencies
COPY stripe-webhooks/go.mod .
COPY stripe-webhooks/go.sum .

WORKDIR /app/lib
COPY lib .
WORKDIR /app/stripe-webhooks
# Download the dependencies - this can be cached unless
# go.mod or go.sum change
RUN go mod download

# Copy the source code into the container
COPY stripe-webhooks/app .
COPY stripe-webhooks/apperror .
COPY stripe-webhooks/middleware .
COPY stripe-webhooks/postgres .
COPY stripe-webhooks/routes .
COPY stripe-webhooks/utils .
COPY stripe-webhooks/main.go .

# Run air for hot reloading
CMD ["air"]

