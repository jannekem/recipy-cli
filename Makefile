NAME = "recipy-cli"

.PHONY: build
build:
	@echo "Building..."
	cargo build --release

.PHONY: install
install: build
	@echo "Installing..."
	sudo install -m 755 target/release/$(NAME) /usr/local/bin/$(NAME)

.PHONY: clean
clean:
	@echo "Cleaning..."
	cargo clean
