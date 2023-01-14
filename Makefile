install:
	sudo dnf install \
		curl \
		llvm \
		cmake \
		binutils \
		clang \
		qemu-user \
		musl-gcc \
		openssl-devel \
		pkg-config

	sudo dnf groupinstall "Development Tools" "Development Libraries"

build:
	podman build -t alvidir/agora:latest -f ./container/agora/containerfile .

deploy:
	podman-compose  -f compose.yaml up --remove-orphans -d

follow:
	podman logs --follow --names agora-server
	
undeploy:
	podman-compose -f compose.yaml down

run:
	RUST_LOG=INFO cargo run

test:
	RUST_BACKTRACE=full cargo test -- --nocapture
