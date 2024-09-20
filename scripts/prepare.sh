#!/bin/sh
set -eu

ADDITIONAL_OPTIONS=""
if [ $# -eq 1 ] && [ "$1" == "--no-cache" ]; then
  ADDITIONAL_OPTIONS="--no-cache"
fi

# Sets: PROJECT_DIR, DEV_IMAGE_NAME, DEV_IMAGE_LOCAL_NAME, ARTIFACTS_DIR
SCRIPT_DIR="$(realpath $(dirname $0))"
. ${SCRIPT_DIR}/vars.sh

# TODO::
# - [ ] `.` here is the "Build Context" (see: https://docs.podman.io/en/latest/markdown/podman-build.1.html#description):
#   Find out what actually happens with this context..?
# - [ ] Manifest file for uploading multiplatform images to a registry created, not uploading anywhere ...
#   - [ ] `podman push ...` or `podman manifest push ...` for uploading to a registry multiplatform images.
# - [ ] Check in registry iff already have a "good" image, and by default do not recreate.

# Delete any old manifest for latest (since its mutable state)
echo a $DEV_IMAGE_MANIFEST
if podman manifest exists ${DEV_IMAGE_MANIFEST}; then
  echo b
  podman manifest rm ${DEV_IMAGE_MANIFEST}
fi
echo c

podman manifest create ${DEV_IMAGE_BASE_NAME}
echo d

podman build --file="${SCRIPT_DIR}/Dockerfile" \
  --platform=linux/amd64,linux/aarch64 \
  --manifest ${DEV_IMAGE_MANIFEST} \
  ${ADDITIONAL_OPTIONS} \
  .
echo e
