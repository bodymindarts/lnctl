build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

clippy:
	cargo clippy --all-features
