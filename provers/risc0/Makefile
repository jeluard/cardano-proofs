setup:
	curl -L https://risczero.com/install | bash
	/home/runner/.risc0/bin/rzup install
	cargo build --release

prove:
	RUST_LOG="[executor]=info" cargo run --release