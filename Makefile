.PHONY: help build release test clean install config demo

# Default target
help:
	@echo "Trans CLI - Baidu Translation Tool"
	@echo ""
	@echo "Available targets:"
	@echo "  build     - Build the project in debug mode"
	@echo "  release   - Build the project in release mode"
	@echo "  test      - Run tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install the binary to ~/.cargo/bin"
	@echo "  config    - Show config file location"
	@echo "  demo      - Run demonstration script"
	@echo "  help      - Show this help message"

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install the binary
install: release
	cp target/release/trans-cli ~/.cargo/bin/
	@echo "trans-cli installed to ~/.cargo/bin/"
	@echo "Make sure ~/.cargo/bin is in your PATH"

# Show config file location
config:
	cargo run -- --config

# Run demonstration
demo:
	./test.sh

# Quick translation example (requires configured API keys)
example:
	@echo "Example translation (requires API credentials):"
	@echo "cargo run -- \"Hello World\""