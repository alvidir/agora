VERSION=0.1.0
PROJECT=agora
REPO=alvidir

build:
	podman build -t ${REPO}/${PROJECT}:${VERSION} -f ./docker/agora/dockerfile .

deploy:
	podman-compose -f docker-compose.yaml up --remove-orphans -d

undeploy:
	podman-compose -f docker-compose.yaml down

purge: undeploy
	podman volume rm --force agora_alpha-data agora_zero-data

reboot: purge deploy

run:
	go run cmd/agora/main.go

test:
	go test -v -race ./... -tags=all

migrate:
	# pip3 install python-dotenv
	python3 scripts/migrate_schema.py
