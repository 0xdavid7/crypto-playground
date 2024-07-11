.PHONY: test test-lib

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
test-zkp:
	RUST_BACKTRACE=1 cargo test --package zkp --lib -- --show-output