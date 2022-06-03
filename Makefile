# storage Makefile

CAPABILITY_ID = "wasmcloud:example:storage"
NAME = "storage"
VENDOR = "Embe"
PROJECT = storage
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

TEST_FLAGS := --release -- --nocapture

test::
	cargo clippy --all-targets --all-features
	cargo test $(TEST_FLAGS)
