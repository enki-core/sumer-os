#!/bin/bash
# سكريبت بناء رسام كيسبو لنظام سومر

set -e

echo "[*] Cleaning..."
cargo clean

echo "[*] Building Kispu Painter..."
cargo build --release

cp target/release/kispu-painter .
echo "[+] Done! Binary is at programs/kispu-painter/kispu-painter"