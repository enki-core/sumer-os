#!/bin/bash
# سكريبت بناء مدير ملفات أطلس لنظام سومر

set -e

echo "[*] Cleaning..."
cargo clean

echo "[*] Building Atlas File Manager..."
cargo build --release

cp target/release/atlas-filemanager .
echo "[+] Done! Binary is at programs/atlas-filemanager/atlas-filemanager"