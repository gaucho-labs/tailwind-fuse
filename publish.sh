#!/usr/bin/env bash

cargo publish --manifest-path ./variant-macro/Cargo.toml $@

cargo publish --manifest-path ./fuse/Cargo.toml $@