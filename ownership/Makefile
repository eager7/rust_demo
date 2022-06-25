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
	RUST_BACKTRACE=1 cargo run

clean:
	-rm -rf build target Cargo.*