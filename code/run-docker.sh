#!/usr/bin/env bash
set -euo pipefail
IMAGE_NAME="${IMAGE_NAME:-deepsurf}"
IMAGE_TAG="${IMAGE_TAG:-latest}"
PLATFORM="${PLATFORM:-linux/amd64}"

docker run --rm -it \
  --platform "${PLATFORM}" \
  -v "$PWD":/workspace -w /workspace \
  "${IMAGE_NAME}:${IMAGE_TAG}" bash