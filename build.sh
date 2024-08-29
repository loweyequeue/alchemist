#!/bin/sh

# FIXME: this is just a temporary file because its midnight and I will forget

docker run -v .:/var/src dev /root/.cargo/bin/cargo build --release
