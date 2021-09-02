# ==============================================================================
# Modules support
SHELL := /bin/bash

gen:
	protoc --go_out=go-src/pb --go_opt=paths=import \
		--go-grpc_out=go-src/pb --go-grpc_opt=paths=import \
		proto/users.proto

server:
	go run go-src/cmd/main.go

clean:
	rm go-src/pb/*.go

tidy:
	go mod tidy

.PHONY: gen server clean tidy
