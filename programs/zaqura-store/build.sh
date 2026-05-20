#!/bin/bash
# سكريبت تجميع متجر زقورة لنظام سومر OS (Alpine Linux)

cd "$(dirname "$0")"

echo "==========================================="
echo "     Compiling Zaqura Store for Alpine...  "
echo "==========================================="

# بناء الباينري باستخدام حاوية Docker الرسمية لـ Rust Alpine
docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release"

# حذف ملف الباينري القديم أولاً ثم نسخ الجديد المجمع إلى جذر المجلد ليسهل تشغيله
rm -f zaqura-store
cp target-alpine/release/zaqura-store .

echo "[+] Zaqura Store compiled successfully!"
