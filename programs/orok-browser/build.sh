#!/bin/sh
set -e

# سكريبت تجميع متصفح أوروك لإنتاج باينري متوافق مع نظام Alpine Linux المصغر
cd "$(dirname "$0")"

echo "==========================================="
echo "      Compiling Orok Browser... "
echo "==========================================="

# محاولة البناء باستخدام Docker لبيئة Alpine
if docker run --rm \
  -v "$PWD:/volume" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/registry" \
  -v "$PWD/../../.cache/cargo_cache:/root/.cargo/git" \
  -w /volume \
  rust:alpine \
  sh -c "apk add --no-cache musl-dev build-base && RUSTFLAGS='-C target-feature=-crt-static' CARGO_TARGET_DIR=target-alpine cargo build --release" 2>/dev/null; then

    echo "[+] Docker Alpine build succeeded!"
    # حذف ملف الباينري القديم أولاً ثم نسخ الجديد المجمع إلى جذر المجلد
    rm -f orok-browser
    cp target-alpine/release/orok-browser .
    echo "[+] Alpine Binary is at programs/orok-browser/orok-browser"

else
    echo "[!] Docker failed or permission denied. Falling back to native host build..."
    cargo build --release
    
    rm -f orok-browser
    cp target/release/orok-browser .
    echo "[+] Native Host Binary is at programs/orok-browser/orok-browser"
fi

echo "[+] Done!"