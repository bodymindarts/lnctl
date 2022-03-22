build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test --all-features -- --nocapture'

run-server: build
	RUST_BACKTRACE=full cargo run server

integration: build
	RUST_BACKTRACE=1 bats -t -r test/integration

clippy:
	cargo clippy --all-features

.PHONY: test integration clippy
