#!/bin/sh

# FIXME: this is just a temporary file because its midnight and I will forget

podman run --arch amd64 -v $(pwd):/var/src --rm dev /root/.cargo/bin/cargo build --release
