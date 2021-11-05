VERSION=0.1.0
PROJECT=agora
REPO=alvidir

build:
	podman build -t ${REPO}/${PROJECT}:${VERSION} -f ./docker/agora/dockerfile .

deploy:
	podman-compose -f docker-compose.yaml up --remove-orphans
	# delete -d in order to see output logs

undeploy:
	podman-compose -f docker-compose.yaml down

run:
	go run cmd/agora/main.go

test:
	go test -v -race ./...

migrate:
	# pip3 install python-dotenv
	python3 scripts/migrate_schema.py