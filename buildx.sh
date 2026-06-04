#!/usr/bin/env bash
set -Eeo pipefail

push="${push:-false}"
repo="${repo:-dyrnq}"
image_name="${image_name:-runlikers}"
platforms="${platforms:-linux/amd64,linux/arm64,linux/arm/v7}"
tag="${tag:-}"

while [ $# -gt 0 ]; do
    case "$1" in
        --push)
            push="$2"
            shift 2
            ;;
        --repo)
            repo="$2"
            shift 2
            ;;
        --image-name|--image)
            image_name="$2"
            shift 2
            ;;
        --platforms)
            platforms="$2"
            shift 2
            ;;
        --tag)
            tag="$2"
            shift 2
            ;;
        --*)
            echo "Illegal option $1"
            exit 1
            ;;
    esac
done

if [ -z "$tag" ]; then
    echo "error: --tag is required"
    exit 1
fi

docker buildx build \
  --platform "${platforms}" \
  --output "type=image,push=${push}" \
  --file ./Dockerfile \
  --tag "${repo}/${image_name}:${tag}" \
  .
