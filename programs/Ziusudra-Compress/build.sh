#!/bin/sh
set -e

cd "$(dirname "$0")"

echo "==========================================="
echo "  Compiling Ziusudra Compress for Alpine...           "
echo "==========================================="

docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

rm -f Ziusudra-Compress
cp target-alpine/release/Ziusudra-Compress .

echo "[+] Ziusudra Compress compiled successfully!"
