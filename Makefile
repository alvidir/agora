install:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
	pip3 install python-dotenv

proto:
	protoc --proto_path=proto --go_out=proto --go_opt=paths=source_relative \
		--go-grpc_out=proto --go-grpc_opt=paths=source_relative \
		proto/*.proto

	go mod tidy

deploy:
	podman-compose -f compose.yaml up

undeploy:
	podman-compose -f compose.yaml down

run:
	go run cmd/filebrowser/main.go

run-mqworkers:
	go run cmd/mq-users/main.go

test:
	go test -v -race ./...

migrate:
	python3 scripts/migrate_schema.py
