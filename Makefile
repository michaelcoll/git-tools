build:
	cargo build

.PHONY: test
test:
	cargo test

dep-upgrade:
	cargo fetch