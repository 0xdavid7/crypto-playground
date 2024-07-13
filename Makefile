.PHONY: test test-lib

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
test-zkp:
ifdef t
	RUST_BACKTRACE=1 cargo test --package zkp --lib -- $(t) --nocapture
else
	RUST_BACKTRACE=1 cargo test --package zkp --lib -- --show-output
endif