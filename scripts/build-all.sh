#!/bin/bash
set -e

# سكريبت لتجميع كافة البرامج والواجهات دفعة واحدة ووضع ملفات الباينري النهائية 
# بداخل المجلد الجذري لكل تطبيق لتسهيل رفعها وتحديثها لاحقاً.

cd "$(dirname "$0")/.."

echo "==========================================="
echo "      SUMER OS - BATCH COMPILER            "
echo "==========================================="

# قائمة بالمجلدات المطلوب تجميعها
TARGET_DIRS=("programs/ur-desktop" "programs/tigris-shell" "programs/nahr-tv" "programs/alwah-notepad" "programs/zaqura-store" "programs/enki-calc")

for DIR in "${TARGET_DIRS[@]}"; do
    if [ -d "$DIR" ]; then
        APP_NAME=$(basename "$DIR")
        echo -e "\n\e[32m[+] البدء بتجميع: $APP_NAME ($DIR)...\e[0m"
        
        # التأكد من وجود سكريبت البناء المحلي وجعله قابلاً للتنفيذ
        if [ -f "$DIR/build.sh" ]; then
            chmod +x "$DIR/build.sh"
            
            # تشغيل سكريبت التجميع المحلي للمشروع
            "$DIR/build.sh"
            
            # تحديد مسار الباينري الناتج بوضع Alpine
            RELEASE_BINARY="$DIR/target-alpine/release/$APP_NAME"
            
            if [ -f "$RELEASE_BINARY" ]; then
                # نسخ الباينري إلى المجلد الجذري للمشروع بنفس الاسم
                FINAL_DEST="$DIR/$APP_NAME"
                cp "$RELEASE_BINARY" "$FINAL_DEST"
                chmod +x "$FINAL_DEST"
                
                echo -e "\e[36m[✓] تم وضع الباينري بنجاح في: $FINAL_DEST\e[0m"
            else
                echo -e "\e[31m[!] خطأ: لم يتم العثور على الباينري المجمع في $RELEASE_BINARY\e[0m"
            fi
        else
            echo -e "\e[33m[!] تحذير: لا يوجد سكريبت build.sh بداخل مجلد $DIR\e[0m"
        fi
    fi
done

echo -e "\n==========================================="
echo -e "\e[32m[+] اكتمل تجميع كافة البرامج والواجهات بنجاح! 🎉\e[0m"
echo -e "==========================================="
