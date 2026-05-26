#!/bin/sh
set -e

# سكريبت تجميع مشغل الموسيقى رنيم لإنتاج باينري متوافق مع نظام Alpine Linux المصغر
cd "$(dirname "$0")"

echo "==========================================="
echo "       Compiling Raneem MP3 for Alpine...  "
echo "==========================================="

docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

# حذف ملف الباينري القديم أولاً ثم نسخ الجديد المجمع إلى جذر المجلد
rm -f raneem-mp3
cp target-alpine/release/raneem-mp3 .

echo "[+] Raneem MP3 compiled successfully!"
