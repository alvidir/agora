BINARY_NAME=agora
VERSION?=latest
PKG_MANAGER?=dnf

all: binaries

binaries: install-deps
ifdef target
	cargo build --bin $(target) --features $(target) --release
else
	-cargo build --bin grpc --features grpc --release
endif

images:
ifdef target
	podman build -t alvidir/$(BINARY_NAME):$(VERSION)-$(target) -f ./container/$(target)/containerfile .
else
	-podman build -t alvidir/$(BINARY_NAME):$(VERSION)-grpc -f ./container/grpc/containerfile .
endif

push-images:
ifdef target
	@podman push alvidir/$(BINARY_NAME):$(VERSION)-$(target)
else
	@-podman push alvidir/$(BINARY_NAME):$(VERSION)-grpc
endif

install-deps:
	-$(PKG_MANAGER) install -y protobuf-compiler
	-$(PKG_MANAGER) install -y postgresql-devel
	-$(PKG_MANAGER) install -y openssl-devel
	-$(PKG_MANAGER) install -y pkg-config

clean:
	@-cargo clean
	@-rm -rf bin/                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     o
	@-rm -rf secrets/

clean-images:
	@-podman image rm alvidir/$(BINARY_NAME):$(VERSION)-grpc
	
test:
	@RUST_BACKTRACE=full cargo test -- --nocapture

deploy:
	@podman-compose -f compose.yaml up -d

undeploy:
	@podman-compose -f compose.yaml down