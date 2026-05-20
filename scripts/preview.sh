#!/bin/bash
# سكريبت استعراض ومعاينة واجهة سومر OS التفاعلية حياً ومباشرة
# دون الحاجة لأي عمليات تجميع أو إقلاع QEMU

cd "$(dirname "$0")/.."

export DISPLAY=:0
# إجبار محرك Slint على تشغيل النافذة بوضع ملء الشاشة الكامل
export SLINT_FULLSCREEN=1

echo "[+] جاري بدء المعاينة الحية لـ Ur Desktop في وضع ملء الشاشة..."
echo "[*] افتح ملف programs/ur-desktop/ui/app.slint وعدّله، وستظهر التغييرات فوراً هنا!"

./programs/slint-viewer/slint-viewer --auto-reload programs/ur-desktop/ui/app.slint
