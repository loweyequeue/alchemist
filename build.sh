#!/bin/sh

# FIXME: this is just a temporary file because its midnight and I will forget

podman run -v .:/var/src dev /root/.cargo/bin/cargo build --release
