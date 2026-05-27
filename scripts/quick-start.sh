#!/bin/bash
# 🚀 سكريبت البدء السريع - Quick Start Script

set -e

COLOR_GREEN='\033[0;32m'
COLOR_BLUE='\033[0;34m'
COLOR_YELLOW='\033[1;33m'
COLOR_RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${COLOR_BLUE}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║  🚀 Sumer OS - نظام التكامل المتقدم                      ║"
echo "║  Advanced Integration System - Quick Start                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# الانتقال إلى المشروع
PROJECT_DIR="/home/debian/Desktop/Sumer-OS-Portable/programs/ur-desktop"

if [ ! -d "$PROJECT_DIR" ]; then
    echo -e "${COLOR_RED}❌ خطأ: المشروع غير موجود في $PROJECT_DIR${NC}"
    exit 1
fi

cd "$PROJECT_DIR"
echo -e "${COLOR_GREEN}✓ تم الانتقال إلى: $PROJECT_DIR${NC}"

# قائمة الخيارات
echo -e "\n${COLOR_BLUE}اختر الخيار:${NC}"
echo "1) البناء (Build)"
echo "2) التشغيل (Run)"
echo "3) الاختبارات (Test)"
echo "4) البناء + التشغيل (Build & Run)"
echo "5) الاختبارات مع التصحيح (Debug Tests)"
echo "6) عرض التوثيق (Documentation)"
echo "0) خروج (Exit)"

read -p "أدخل رقم الخيار: " choice

case $choice in
    1)
        echo -e "\n${COLOR_YELLOW}🔨 جاري البناء...${NC}"
        cargo build --release
        echo -e "${COLOR_GREEN}✓ تم البناء بنجاح!${NC}"
        ;;
    2)
        echo -e "\n${COLOR_YELLOW}🚀 جاري التشغيل...${NC}"
        cargo run --release
        ;;
    3)
        echo -e "\n${COLOR_YELLOW}🧪 جاري الاختبارات...${NC}"
        cargo test
        echo -e "${COLOR_GREEN}✓ تمت الاختبارات بنجاح!${NC}"
        ;;
    4)
        echo -e "\n${COLOR_YELLOW}🔨 جاري البناء + التشغيل...${NC}"
        cargo build --release && cargo run --release
        ;;
    5)
        echo -e "\n${COLOR_YELLOW}🧪 جاري الاختبارات مع التصحيح...${NC}"
        RUST_LOG=debug cargo test -- --nocapture
        ;;
    6)
        echo -e "\n${COLOR_BLUE}📖 ملفات التوثيق:${NC}"
        echo ""
        echo "1) INTEGRATION_GUIDE.md - شرح شامل (300+ سطر)"
        echo "2) ../../../INTEGRATION_README.md - دليل سريع"
        echo "3) ../../../COMPLETION_SUMMARY.md - ملخص الإنجازات"
        echo "4) ../../../ARCHITECTURE_DIAGRAM.txt - الهيكلة المعمارية"
        echo "5) PRISM_IPC_EXAMPLE.rs - أمثلة الاستخدام"
        echo ""
        
        read -p "أيهما تريد أن تقرأ؟ (1-5): " doc_choice
        
        case $doc_choice in
            1) less INTEGRATION_GUIDE.md ;;
            2) less ../../../INTEGRATION_README.md ;;
            3) less ../../../COMPLETION_SUMMARY.md ;;
            4) less ../../../ARCHITECTURE_DIAGRAM.txt ;;
            5) less ../prism-screenshot/PRISM_IPC_EXAMPLE.rs ;;
            *) echo -e "${COLOR_RED}❌ اختيار غير صحيح${NC}" ;;
        esac
        ;;
    0)
        echo -e "${COLOR_GREEN}👋 وداعاً!${NC}"
        exit 0
        ;;
    *)
        echo -e "${COLOR_RED}❌ اختيار غير صحيح${NC}"
        exit 1
        ;;
esac

echo -e "\n${COLOR_GREEN}✅ تمت العملية بنجاح!${NC}"
