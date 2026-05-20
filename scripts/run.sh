#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# سكريبت التشغيل الموحد لنظام سومر
echo "==========================================="
echo "      SUMER OS - BUILD SYSTEM              "
echo "==========================================="

# تنفيذ سكريبت البناء الأساسي
sudo ./scripts/build-gui-rootfs.sh

echo ""
echo "[+] عملية البناء انتهت بنجاح."
echo "[+] يمكنك الآن التشغيل باستخدام: ./scripts/launch.sh"
