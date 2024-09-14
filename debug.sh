#!/bin/sh

SCRIPT_DIR="$(realpath $(dirname $0))"
#PROJECT_DIR=$(dirname $SCRIPT_DIR)
source "${SCRIPT_DIR}/vars.sh"

if [ "$1" == "--keep" ]; then
  podman run --arch amd64 -it --rm -v $(pwd):/var/src "${DEV_IMAGE_NAME}" /bin/sh
else
  podman run --arch amd64 -it -v $(pwd):/var/src "${DEV_IMAGE_NAME}" /bin/sh
fi
