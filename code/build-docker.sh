#!/usr/bin/env bash
set -euo pipefail

# Locate repo root (one level up from this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

IMAGE_NAME="${IMAGE_NAME:-deepsurf}"
IMAGE_TAG="${IMAGE_TAG:-latest}"
PLATFORM="${PLATFORM:-linux/amd64}"

# UPDATED default: point to your new Dockerfile path (edit if needed)
DOCKERFILE="${DOCKERFILE:-${REPO_ROOT}/code/Dockerfile}"
CONTEXT="${CONTEXT:-${REPO_ROOT}}"

NO_CACHE="${NO_CACHE:-0}"
PULL="${PULL:-0}"

command -v docker >/dev/null || { echo "[ERROR] docker not found" >&2; exit 1; }
[[ -f "${DOCKERFILE}" ]] || { echo "[ERROR] Dockerfile not found: ${DOCKERFILE}" >&2; exit 1; }

ARGS=( --platform "${PLATFORM}" -f "${DOCKERFILE}" -t "${IMAGE_NAME}:${IMAGE_TAG}" )
[[ "${NO_CACHE}" == "1" ]] && ARGS+=( --no-cache )
[[ "${PULL}" == "1" ]] && ARGS+=( --pull )

echo "[build] Context:    ${CONTEXT}"
echo "[build] Dockerfile: ${DOCKERFILE}"
echo "[build] Image:      ${IMAGE_NAME}:${IMAGE_TAG}"
echo "[build] Platform:   ${PLATFORM}"

DOCKER_BUILDKIT="${DOCKER_BUILDKIT:-1}" docker build "${ARGS[@]}" "${CONTEXT}"

echo "[build] Done: ${IMAGE_NAME}:${IMAGE_TAG}"