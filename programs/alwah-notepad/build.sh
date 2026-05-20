#!/bin/sh
set -e

# سكريبت تجميع مفكرة ألواح لإنتاج باينري متوافق مع نظام Alpine Linux المصغر
cd "$(dirname "$0")"

echo "==========================================="
echo "     Compiling Alwah Notepad for Alpine... "
echo "==========================================="

docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

# حذف ملف الباينري القديم أولاً ثم نسخ الجديد المجمع إلى جذر المجلد
rm -f alwah-notepad
cp target-alpine/release/alwah-notepad .

echo "[+] Alwah Notepad compiled successfully!"
