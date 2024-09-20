#!/bin/sh
# TODO: re-enable below, and fix wrong-arch-container chosen, because ...?
#set -eu

# Sets: PROJECT_DIR, DEV_IMAGE_NAME, DEV_IMAGE_LOCAL_NAME, ARTIFACTS_DIR
SCRIPT_DIR="$(realpath $(dirname $0))"
. "${SCRIPT_DIR}/vars.sh"

# Ensure environment is clean:
test -d "${ARTIFACTS_DIR}" && rm -rf "${ARTIFACTS_DIR}"
test -d target && rm target
mkdir -p target-x86-cache
mkdir -p target-aarch64-cache
mkdir -p target-host-cache
mkdir -p "${ARTIFACTS_DIR}"

echo
echo '=================='
echo '= AMD / INTEL 64 ='
echo '=================='
echo
ln -s target-x86-cache target
podman run --arch=amd64 -v ${PROJECT_DIR}:/var/src --rm registry.localhost8080.org/alchemist-build:latest cargo build --release
if test -f ${PROJECT_DIR}/target/release/alchemist; then
  cp -p ${PROJECT_DIR}/target/release/alchemist ${ARTIFACTS_DIR}/alchemist-linux-x86_64
  rm target
else
  rm target
  ln -s target-host-cache target
  echo "No build artifact found, aborting"
  exit 1
fi

echo
echo '=================='
echo '=    AARCH 64    ='
echo '=================='
echo
ln -s target-aarch64-cache target
podman run --arch=arm64 -v ${PROJECT_DIR}:/var/src --rm registry.localhost8080.org/alchemist-build:latest cargo build --release
if test -f ${PROJECT_DIR}/target/release/alchemist; then
  cp -p ${PROJECT_DIR}/target/release/alchemist ${ARTIFACTS_DIR}/alchemist-linux-aarch64
  rm target
else
  rm target
  ln -s target-host-cache target
  echo "No build artifact found, aborting"
  exit 1
fi

ln -s target-host-cache target
