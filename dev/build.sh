#!/usr/bin/env bash

set -euxo pipefail

if type "podman" > /dev/null; then
  echo "Found podman"
  podman build -f Dockerfile --squash -t irox-dev:latest .
elif type "docker" > /dev/null; then
  echo "Found docker"
  docker build -f Dockerfile -t irox-dev:latest .
else
  echo "Could not find podman nor docker on the path"
  exit 1
fi

