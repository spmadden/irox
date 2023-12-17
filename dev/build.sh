#!/usr/bin/env bash

set -euxo pipefail

VSN=`git describe --dirty`
TAGS="-t ghcr.io/spmadden/irox-dev:latest -t ghcr.io/spmadden/irox-dev:$VSN"

if type "podman" > /dev/null; then
  echo "Found podman"
  podman build -f Dockerfile --squash $TAGS .
elif type "docker" > /dev/null; then
  echo "Found docker"
  docker build -f Dockerfile $TAGS .
else
  echo "Could not find podman nor docker on the path"
  exit 1
fi

