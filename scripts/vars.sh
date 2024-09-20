# WARNING: calling script must have at least `set -u` (nounset) set.
PROJECT_DIR=$(dirname $SCRIPT_DIR)
DEV_IMAGE_BASE_NAME="alchemist-build"
DEV_IMAGE_NAME="${DEV_IMAGE_BASE_NAME}:latest"
DEV_IMAGE_MANIFEST="localhost/${DEV_IMAGE_BASE_NAME}"
DEV_IMAGE_LOCAL_NAME="localhost/${DEV_IMAGE_NAME}"
ARTIFACTS_DIR="$PROJECT_DIR/artifacts"
