.PHONY: build test lint fmt fmt-check ci clean

# Build the binary
build:
	cargo build --release
	mkdir -p bin
	cp target/release/ralph-hook-fmt bin/

# Run tests (integration tests run sequentially to avoid cargo build conflicts)
test:
	cargo test --bins
	cargo test --test integration -- --test-threads=1

# Run clippy
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check

# Run all CI checks
ci: fmt-check lint test

# Clean build artifacts
clean:
	cargo clean
	rm -rf bin/
