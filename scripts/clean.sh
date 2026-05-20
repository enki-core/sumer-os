#!/bin/bash
# سكريبت تنظيف مساحة العمل وحذف ملفات البناء المؤقتة ومجلدات target المترجمة
# لتوفير مساحة على القرص والبدء ببناء جديد كلياً عند الحاجة

cd "$(dirname "$0")/.."

echo "==========================================="
echo "      SUMER OS - WORKSPACE CLEANER         "
echo "==========================================="

echo "[*] جاري حساب المساحة المستهلكة حالياً..."
FREED_BEFORE=$(du -sh . 2>/dev/null | cut -f1)
echo "[+] المساحة الحالية للمجلد: $FREED_BEFORE"

echo -e "\n[*] جاري حذف مجلدات target ونواتج التجميع المؤقتة..."

# حذف مجلدات target في مجلدات Rust
rm -rf programs/ur-desktop/target programs/ur-desktop/target-alpine
rm -rf programs/tigris-shell/target programs/tigris-shell/target-alpine

# حذف مجلدات البناء والـ ISO المؤقتة
rm -rf build_tmp
rm -rf iso/*

# حذف كاش apk وحزم البناء إذا كانت موجودة لتوفير أقصى مساحة
# rm -rf .cache # إلغاء التعليق عند الرغبة في حذف الكاش بالكامل

echo "[+] تم مسح ملفات التجميع المؤقتة بنجاح!"

FREED_AFTER=$(du -sh . 2>/dev/null | cut -f1)
echo "[+] مساحة المجلد بعد التنظيف: $FREED_AFTER"
echo "[+] تم تنظيف مساحة العمل بنجاح والبدء بصفحة بيضاء! 🧹✨"
