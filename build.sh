#!/bin/sh

SCRIPT_DIR="$(realpath $(dirname $0))"
#PROJECT_DIR=$(dirname $SCRIPT_DIR)
source "${SCRIPT_DIR}/vars.sh"
podman run --arch amd64 -v $(pwd):/var/src --rm "${DEV_IMAGE_NAME}" cargo build --release
