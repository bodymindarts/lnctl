build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test --all-features -- --nocapture'

run-server: build
	RUST_BACKTRACE=full cargo run server

connector-integration:
	RUST_BACKTRACE=1 bats -t -r test/connector

gateway-integration:
	RUST_BACKTRACE=1 bats -t -r test/gateway

client-integration:
	RUST_BACKTRACE=1 bats -t -r test/client

integration: build connector-integration gateway-integration client-integration

clippy:
	cargo clippy --all-features

test-in-ci: clippy
	cargo test --all-features --verbose --locked

.PHONY: test integration clippy
