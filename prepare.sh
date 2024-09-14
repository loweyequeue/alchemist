#!/bin/sh

SCRIPT_DIR="$(realpath $(dirname $0))"
#PROJECT_DIR=$(dirname $SCRIPT_DIR)
source "${SCRIPT_DIR}/vars.sh"
podman build --arch amd64 -t ${DEV_IMAGE_NAME} .
