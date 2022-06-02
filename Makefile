# storage Makefile

CAPABILITY_ID = "wasmcloud:example:storage"
NAME = "storage"
VENDOR = "Embe"
PROJECT = storage
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

test::
	cargo clippy --all-targets --all-features

