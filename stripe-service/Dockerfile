# Start from the official Golang image for a specific version
FROM golang:1.22 as local

WORKDIR /app

# Install air for hot reloading
RUN go install github.com/cosmtrek/air@latest

# Copy go.mod and go.sum to download the dependencies
COPY go.mod .
COPY go.sum .

# Download the dependencies - this can be cached unless
# go.mod or go.sum change
RUN go mod download

# Copy the source code into the container
COPY main.go .
COPY app .
COPY auth .
COPY postgres .
COPY routes .

# Run air for hot reloading
CMD ["air"]

