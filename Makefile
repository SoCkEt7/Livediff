.PHONY: help build test lint fmt clean

help:
	@echo "Available commands:"
	@echo "  build    - Build the project"
	@echo "  test     - Run tests"
	@echo "  lint     - Run clippy for linting"
	@echo "  fmt      - Format code"
	@echo "  clean    - Clean the project"

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy -- -D clippy::perf

fmt:
	cargo fmt

clean:
	cargo clean
