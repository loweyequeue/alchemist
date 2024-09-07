#!/bin/sh

podman run --arch amd64 -it -v $(pwd):/var/src dev /bin/sh
