#!/bin/bash
cd "$(dirname "$0")"

# بناء التطبيق أولاً لضمان تحديث ملفات Slint
./build.sh

# ثم تشغيل التطبيق
./orok-browser
