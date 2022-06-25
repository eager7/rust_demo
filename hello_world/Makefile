#/bin/bash

SRC_ROOT := $(shell pwd)

.PHONY: default clean run init target

default:run

target:
	cargo build

# Create a new cargo package in an existing directory
init:
	cargo init $(SRC_ROOT)

run:
	cargo run

test:
	cargo test -- --show-output --test-threads=1 --include-ignored
test_ignore:
	cargo test -- --show-output --test-threads=1 --ignored

clean:
	-rm -rf build target Cargo.*