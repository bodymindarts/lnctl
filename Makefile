build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test --all-features -- --nocapture'

run-server: build
	RUST_BACKTRACE=full cargo run server

connector-integration:
	RUST_BACKTRACE=1 bats -t -r test/connector

coordinator-integration:
	RUST_BACKTRACE=1 bats -t -r test/coordinator

client-integration:
	RUST_BACKTRACE=1 bats -t -r test/client

integration: build connector-integration coordinator-integration client-integration

clippy:
	cargo clippy --all-features

.PHONY: test integration clippy
