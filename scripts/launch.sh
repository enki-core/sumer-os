#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo -e "\e[32m[+] Launching Sumer OS (Universal Debug Mode)...\e[0m"

echo "[+] Launching QEMU..."
rm -f qemu_serial.log
touch qemu_serial.log

# إنشاء قرص وهمي للذاكرة الدائمة (إذا لم يكن موجوداً) بحجم 200 ميغابايت بصيغة ext4 مع تسميته بـ SUMER-DATA
if [ ! -f "persist.img" ]; then
    echo "[+] Creating Persistence Virtual Disk (persist.img)..."
    dd if=/dev/zero of=persist.img bs=1M count=200 status=none
    /sbin/mkfs.ext4 -F -L SUMER-DATA persist.img >/dev/null 2>&1
    echo "[+] Persistence Disk formatted successfully!"
fi

# تشغيل QEMU مع إضافة القرص الوهمي كذاكرة دائمة
qemu-system-x86_64 -m 2G -cdrom "iso/Sumer-OS.iso" -vga std \
    -usb -device usb-tablet \
    -drive file=persist.img,format=raw,if=virtio \
    -display gtk,zoom-to-fit=on \
    -serial file:qemu_serial.log \
    -monitor telnet:127.0.0.1:4444,server,nowait >/dev/null 2>&1 &

echo "[+] QEMU launched in background with usb-tablet."
exit 0
