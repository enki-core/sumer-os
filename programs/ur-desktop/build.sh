#!/bin/sh
set -e

# سكريبت تجميع واجهة ديسكتاب أور لإنتاج باينري متوافق مع نظام Alpine Linux المصغر
cd "$(dirname "$0")"

echo "==========================================="
echo "  Compiling Ur Desktop for Alpine...       "
echo "==========================================="

docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

# حذف باينري قديم ونسخ الجديد
rm -f ur-desktop sumer-desktop sumer-ui
cp target-alpine/release/ur-desktop .

echo "[+] Ur Desktop compiled successfully!"
