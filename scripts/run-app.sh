#!/bin/bash
set -e

# سكريبت لتشغيل تطبيقات Sumer OS بشكل مباشر ومحلي على الجهاز المضيف (Debian/Ubuntu)
# لتسريع التطوير وتجربة التفاعل والأصوات بشكل فوري دون الحاجة للمحاكي

cd "$(dirname "$0")/.."

export DISPLAY=:0
export SLINT_FULLSCREEN=1

APP_TO_RUN=${1:-"sumer-ui"}

case "$APP_TO_RUN" in
    "tv" | "nahr-tv" | "nahr")
        echo "[+] جاري تجميع وتشغيل واجهة نهر تي في (nahr-tv) محلياً..."
        cargo run --manifest-path programs/nahr-tv/Cargo.toml
        ;;
    "shell" | "tigris" | "tigris-shell")
        echo "[+] جاري تجميع وتشغيل شيل دجلة (tigris-shell) محلياً..."
        cargo run --manifest-path programs/tigris-shell/Cargo.toml
        ;;
    "ur-desktop" | "desktop" | "ur" | *)
        echo "[+] جاري تجميع وتشغيل واجهة ديسكتاب أور (ur-desktop) محلياً..."
        cargo run --manifest-path programs/ur-desktop/Cargo.toml
        ;;
esac

