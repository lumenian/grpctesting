# ==============================================================================
# Modules support
SHELL := /bin/bash

gen:
	protoc --go_out=src-go/pb --go_opt=paths=import \
		--go-grpc_out=src-go/pb --go-grpc_opt=paths=import \
		proto/users.proto

server:
	cd src-go; \
	go run cmd/main.go

clean:
	rm src-go/pb/*.go

tidy:
	go mod tidy

.PHONY: gen server clean tidy
