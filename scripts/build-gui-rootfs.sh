#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# التحقق من وجود صلاحيات root
if [ "$EUID" -ne 0 ]; then
  echo "[!] يرجى تشغيل السكريبت باستخدام sudo: sudo ./run.sh"
  exit 1
fi

ROOTFS="$PWD/gui_rootfs"
BUILD_DIR="$PWD/build_tmp"
INITRAMFS_PATH="$BUILD_DIR/initramfs.img"

echo "[+] البدء ببناء نظام سومر (Sumer OS - Native Edition)..."

# إنشاء مجلدات الكاش
mkdir -p .cache/cargo_cache .cache/apk_cache

# 1. بناء نظام الملفات الأساسي
if [ ! -d "$ROOTFS" ]; then
    mkdir -p $ROOTFS
    echo "[+] Installing Base System (Full Graphics Stack)..."
    docker run --rm -v $ROOTFS:/rootfs -v "$PWD/.cache/apk_cache:/apk_cache" alpine:3.19 sh -c "apk add --root /rootfs --initdb --cache-dir /apk_cache && mkdir -p /rootfs/etc/apk/keys && cp -r /etc/apk/keys/* /rootfs/etc/apk/keys/ && echo 'http://dl-cdn.alpinelinux.org/alpine/v3.19/main' > /rootfs/etc/apk/repositories && echo 'http://dl-cdn.alpinelinux.org/alpine/v3.19/community' >> /rootfs/etc/apk/repositories && apk add --root /rootfs --cache-dir /apk_cache alpine-base eudev dbus wayland cage mesa-dri-gallium mesa-egl mesa-gles mesa-gbm wlroots libxkbcommon seatd linux-lts linux-firmware-none zstd libc6-compat gcompat font-dejavu xkeyboard-config xf86-video-fbdev && chroot /rootfs apk del linux-virt || true && chroot /rootfs depmod -a \$(ls /rootfs/lib/modules | grep lts | head -n 1)"
fi

# 2. بناء واجهة Slint التفاعلية
echo "[+] Building Sumer Native UI (Interactive Version)..."
mkdir -p programs/ur-desktop/ui

# 2. نسخ واجهة سومر المجمعة مسبقاً
echo "[+] Copying Ur Desktop..."
if [ ! -f "programs/ur-desktop/target-alpine/release/ur-desktop" ]; then
    echo -e "\e[31m[!] Error: ur-desktop is not compiled yet. Please run 'programs/ur-desktop/build.sh' first!\e[0m"
    exit 1
fi
mkdir -p $ROOTFS/usr/bin
cp programs/ur-desktop/target-alpine/release/ur-desktop $ROOTFS/usr/bin/ur-desktop
chmod +x $ROOTFS/usr/bin/ur-desktop

# 2.5. نسخ شيل دجلة المجمع مسبقاً
echo "[+] Copying Tigris Shell..."
if [ ! -f "programs/tigris-shell/target-alpine/release/tigris-shell" ]; then
    echo -e "\e[31m[!] Error: tigris-shell is not compiled yet. Please run 'programs/tigris-shell/build.sh' first!\e[0m"
    exit 1
fi
cp programs/tigris-shell/target-alpine/release/tigris-shell $ROOTFS/usr/bin/tigris-shell
chmod +x $ROOTFS/usr/bin/tigris-shell

# 2.6. نسخ واجهة نهر تي في المجمعة مسبقاً
echo "[+] Copying Nahr TV..."
if [ ! -f "programs/nahr-tv/target-alpine/release/nahr-tv" ]; then
    echo -e "\e[31m[!] Error: nahr-tv is not compiled yet. Please run 'programs/nahr-tv/build.sh' first!\e[0m"
    exit 1
fi
cp programs/nahr-tv/target-alpine/release/nahr-tv $ROOTFS/usr/bin/nahr-tv
chmod +x $ROOTFS/usr/bin/nahr-tv

# 2.7. نسخ مفكرة ألواح المجمعة مسبقاً
echo "[+] Copying Alwah Notepad..."
if [ ! -f "programs/alwah-notepad/target-alpine/release/alwah-notepad" ]; then
    echo -e "\e[31m[!] Error: alwah-notepad is not compiled yet. Please run 'programs/alwah-notepad/build.sh' first!\e[0m"
    exit 1
fi
cp programs/alwah-notepad/target-alpine/release/alwah-notepad $ROOTFS/usr/bin/alwah-notepad
chmod +x $ROOTFS/usr/bin/alwah-notepad

# 2.8. نسخ حاسبة إنكي المجمعة مسبقاً
echo "[+] Copying Enki Calc..."
if [ ! -f "programs/enki-calc/target-alpine/release/enki-calc" ]; then
    echo -e "\e[31m[!] Error: enki-calc is not compiled yet. Please run 'programs/enki-calc/build.sh' first!\e[0m"
    exit 1
fi
cp programs/enki-calc/target-alpine/release/enki-calc $ROOTFS/usr/bin/enki-calc
chmod +x $ROOTFS/usr/bin/enki-calc

# 2.8. نسخ متجر زقورة المجمع مسبقاً
echo "[+] Copying Zaqura Store..."
if [ ! -f "programs/zaqura-store/target-alpine/release/zaqura-store" ]; then
    echo -e "\e[31m[!] Error: zaqura-store is not compiled yet. Please run 'programs/zaqura-store/build.sh' first!\e[0m"
    exit 1
fi
cp programs/zaqura-store/target-alpine/release/zaqura-store $ROOTFS/usr/bin/zaqura-store
chmod +x $ROOTFS/usr/bin/zaqura-store



# 3. إعداد بيئة الإقلاع
echo "[+] Configuring Boot Environment..."

# حذف أي آثار قديمة
rm -f $ROOTFS/init $ROOTFS/init_stage2 $ROOTFS/sbin/init_stage2 $ROOTFS/sbin/init

cat << 'EOF' > $ROOTFS/init
#!/bin/sh
export PATH=/sbin:/bin:/usr/sbin:/usr/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
mount -t tmpfs tmpfs /tmp
mount -t tmpfs tmpfs /run

# Start device manager for libinput
echo "[+] Starting udevd..." > /dev/ttyS0
/sbin/udevd --daemon

# Load Input and USB drivers (all host controllers)
echo "[*] Loading input and USB modules..." > /dev/ttyS0
/sbin/modprobe psmouse > /dev/ttyS0 2>&1
/sbin/modprobe uhci-hcd > /dev/ttyS0 2>&1
/sbin/modprobe ohci-pci > /dev/ttyS0 2>&1
/sbin/modprobe ehci-pci > /dev/ttyS0 2>&1
/sbin/modprobe xhci-pci > /dev/ttyS0 2>&1
/sbin/modprobe usbhid > /dev/ttyS0 2>&1


/sbin/udevadm trigger
/sbin/udevadm settle

echo "==========================================" > /dev/ttyS0
echo "   SUMER OS - FRAMEBUFFER MODE ACTIVE     " > /dev/ttyS0
echo "==========================================" > /dev/ttyS0

# ==================================================
# 💾 SUMER OS PERSISTENCE LAYER (OverlayFS)
# ==================================================
ZEN_MODE=0
if grep -q "sumer_zen=1" /proc/cmdline; then
    echo "[!] Immutable Zen Mode requested in boot options. Skipping persistence." > /dev/ttyS0
    ZEN_MODE=1
fi

echo "[*] Checking for Persistence Drive (Label: SUMER-DATA)..." > /dev/ttyS0
/sbin/modprobe ext4 > /dev/ttyS0 2>&1
/sbin/modprobe overlay > /dev/ttyS0 2>&1

# Some USB drives might take a second to register after settle
sleep 2

PERSIST_DEV="/dev/disk/by-label/SUMER-DATA"
if [ "$ZEN_MODE" != "1" ] && [ -e "$PERSIST_DEV" ]; then
    echo "[!] Persistence Drive Found! Mounting..." > /dev/ttyS0
    mkdir -p /mnt/persist
    if mount -t ext4 "$PERSIST_DEV" /mnt/persist; then
        echo "[+] Persistence mounted successfully." > /dev/ttyS0
        
        # Overlay /root (Home Directory for GUI)
        mkdir -p /mnt/persist/root_upper /mnt/persist/root_work
        echo "[*] Activating OverlayFS for /root..." > /dev/ttyS0
        mount -t overlay overlay -o lowerdir=/root,upperdir=/mnt/persist/root_upper,workdir=/mnt/persist/root_work /root
        
        # Overlay /etc (System Configurations)
        mkdir -p /mnt/persist/etc_upper /mnt/persist/etc_work
        echo "[*] Activating OverlayFS for /etc..." > /dev/ttyS0
        mount -t overlay overlay -o lowerdir=/etc,upperdir=/mnt/persist/etc_upper,workdir=/mnt/persist/etc_work /etc
        
        echo "[!] Persistence Layer Active." > /dev/ttyS0
    else
        echo "[!] Failed to mount Persistence Drive." > /dev/ttyS0
    fi
else
    echo "[*] No Persistence Drive found. Running in Live Mode." > /dev/ttyS0
fi
# ==================================================


# Try to load display drivers
echo "[*] Attempting to load graphics modules..." > /dev/ttyS0
/sbin/modprobe bochs_drm > /dev/ttyS0 2>&1
/sbin/modprobe bochs > /dev/ttyS0 2>&1
/sbin/modprobe virtio-gpu > /dev/ttyS0 2>&1
/sbin/modprobe virtio_gpu > /dev/ttyS0 2>&1

if [ ! -e /dev/fb0 ]; then
    echo "[!] fb0 not found, listing available DRM modules:" > /dev/ttyS0
    find /lib/modules -name "*drm*" > /dev/ttyS0
    find /lib/modules -name "*gpu*" > /dev/ttyS0
fi

echo "[*] Waiting for /dev/fb0..." > /dev/ttyS0
for i in 1 2 3 4 5; do
    if [ -e /dev/fb0 ]; then
        echo "[!] Screen Found!" > /dev/ttyS0
        break
    fi
    sleep 1
done

# Start D-Bus
mkdir -p /run/dbus
dbus-daemon --system --fork

# Start seatd for KMS/DRM access
echo "[+] Starting seatd..." > /dev/ttyS0
seatd -g root &
sleep 1
export SEATD_SOCK=/run/seatd.sock

# Launch UI
export SLINT_BACKEND=linuxkms-femtovg
export SLINT_FULLSCREEN=1
echo "[+] Launching Ur Desktop..." > /dev/ttyS0
/usr/bin/ur-desktop >> /dev/ttyS0 2>&1 || {
    echo "[!] UI Failed. Dropping to Recovery Shell." > /dev/ttyS0
    sh
}
EOF

chmod +x $ROOTFS/init

# 4. الحزم النهائي
echo "[+] Packing Sumer-OS ISO..."
mkdir -p build_tmp/iso_build/boot/grub
# العثور على الكرنل المتاح ونسخه
KERNEL_FILE=$(ls $ROOTFS/boot/vmlinuz-lts 2>/dev/null || ls $ROOTFS/boot/vmlinuz-* | head -n 1)
cp $KERNEL_FILE build_tmp/iso_build/boot/vmlinuz
# Optimize rootfs size
echo "[+] Optimizing rootfs size..."
rm -rf $ROOTFS/var/cache/apk/*
find $ROOTFS/usr/bin $ROOTFS/usr/sbin $ROOTFS/usr/lib -type f -exec strip --strip-unneeded {} + 2>/dev/null || true

# Re-pack with maximum compression
(cd $ROOTFS && find . | cpio -o -H newc | gzip -9 > ../build_tmp/iso_build/boot/initramfs.img)

cat << 'EOF' > build_tmp/iso_build/boot/grub/grub.cfg
set default=0
set timeout=3

menuentry "Sumer OS (Normal Mode - Auto-Save)" {
    linux /boot/vmlinuz console=ttyS0 console=tty1 earlyprintk=ttyS0 rdinit=/init
    initrd /boot/initramfs.img
}

menuentry "Sumer OS Guest Mode (Immutable Zen Mode)" {
    linux /boot/vmlinuz console=ttyS0 console=tty1 earlyprintk=ttyS0 rdinit=/init sumer_zen=1
    initrd /boot/initramfs.img
}
EOF

# حذف نسخة ISO القديمة لضمان بناء جديد ونظيف بالكامل
rm -f iso/Sumer-OS.iso

mkdir -p iso
grub-mkrescue -o iso/Sumer-OS.iso build_tmp/iso_build
echo "[!] تم بناء Sumer-OS بنجاح!"
