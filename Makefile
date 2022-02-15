build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

integration: build
	bats -t -r test/integration

clippy:
	cargo clippy --all-features

.PHONY: test integration clippy
