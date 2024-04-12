#!/usr/bin/env bash

cargo check
if [ $? -ne 0 ]; then
    echo "Error: cargo check failed. Publishing aborted."
    exit 1
fi

echo "Publishing variant-macro"
cargo publish --manifest-path ./variant-macro/Cargo.toml $@

echo "Publishing crate"
cargo publish --manifest-path ./fuse/Cargo.toml $@