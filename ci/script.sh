#! /usr/bin/env bash -ex

cargo test --target $TARGET --all && cargo build --release --target $TARGET

exit 0
