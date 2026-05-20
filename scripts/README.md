# 🛠️ Sumer OS Scripting Engine Guide | دليل محرك سكربتات نظام سومر

Welcome to the **Sumer OS** scripting folder! This directory contains the automation scripts designed to build, clean, flash, run, and debug Sumer OS (Native & Portable editions).

مرحباً بك في مجلد سكربتات **نظام سومر OS**! يحتوي هذا المجلد على مجموعة من سكربتات الأتمتة المبرمجة لبناء وتجميع وتنظيف وتشغيل وتثبيت النظام على الأجهزة الحقيقية أو الافتراضية.

---

## 📋 Quick Directory Overview | نظرة سريعة على السكربتات

| Script | English Description | الوصف بالعربية |
| :--- | :--- | :--- |
| **[`run.sh`](./run.sh)** | Main entrypoint to build the ISO and automatically launch it in QEMU. | المدخل الرئيسي لبناء نظام الـ ISO وتشغيله تلقائياً في محاكي QEMU. |
| **[`build-all.sh`](./build-all.sh)** | Loops through all apps, compiles their Alpine binaries, and copies them to their roots. | يمر على كل البرامج، يجمع باينري الـ Alpine الخاص بها، وينسخها لجذور مجلداتها. |
| **[`build-gui-rootfs.sh`](./build-gui-rootfs.sh)** | Prepares the Alpine RootFS, copies compiled binaries, and bundles the final Live ISO. | يجهز نظام الملفات المصغر للـ Live ISO، ينسخ باينري البرامج، ويصنع الـ ISO النهائي. |
| **[`run-app.sh`](./run-app.sh)** | Runs any program (`sumer-ui`, `tv`, `shell`) locally on the host machine for fast testing. | يشغل أي برنامج من برامج النظام محلياً على جهاز المضيف لتسريع البرمجة والتجربة. |
| **[`launch.sh`](./launch.sh)** | Bootstraps the virtual system in QEMU and attaches the persistent `SUMER-DATA` storage disk. | يشغل الـ ISO داخل محاكي QEMU مع ربط قرص حفظ التغييرات التلقائي. |
| **[`clean.sh`](./clean.sh)** | Wipes Cargo target caches and build artifacts to free disk space. | ينظف الكاش المؤقت للمشاريع وملفات البناء القديمة لتحرير مساحة القرص. |
| **[`preview.sh`](./preview.sh)** | Runs the already compiled ISO in QEMU instantly without rebuilding it. | يشغل ملف الـ ISO المبني مسبقاً داخل محاكي QEMU فوراً دون الحاجة لإعادة تجميعه. |
| **[`folder2exe.sh`](./folder2exe.sh)** | Placeholder hook for the future Windows USB flashing utility compiler. | ملف فارغ مهيأ لبناء تطبيق حرق الفلاشة لنظام ويندوز مستقبلاً. |

---

## 🔍 Detailed Script Breakdown | الشرح التفصيلي للسكربتات

### 1️⃣ `run.sh`
*   **English:**
    *   **Purpose:** The master developer script. It acts as the orchestrator. When run, it requests root privileges (`sudo`), calls `build-gui-rootfs.sh` to compile the Live system image, and then automatically launches it using `launch.sh`.
    *   **Usage:** `sudo ./scripts/run.sh`
*   **العربية:**
    *   **الوظيفة:** السكربت الرئيسي للمطورين. يقوم بطلب صلاحيات المسؤول (`sudo`) ثم يقوم باستدعاء سكربت البناء لصنع نظام الـ ISO الجديد وتشغيله تلقائياً في محاكي QEMU بضغطة زر واحدة.
    *   **الاستخدام:** `sudo ./scripts/run.sh`

---

### 1.5️⃣ `build-all.sh`
*   **English:**
    *   **Purpose:** The batch compile script you requested. It automatically loops through the main `sumer-ui/` directory and every active program directory in `programs/` (such as `tigris-shell/` and `android-tv-ui/`). It executes their local `build.sh` compilation scripts inside a Docker Alpine container, and then copies the final compiled Alpine binary into the root of its own project folder with the application name (e.g. `sumer-ui/sumer-ui`, `programs/tigris-shell/tigris-shell`). This lets you easily extract them and push them as GitHub updates!
    *   **Usage:** `./scripts/build-all.sh`
*   **العربية:**
    *   **الوظيفة:** سكريبت التجميع الشامل الذي طلبته. يقوم بالمرور التلقائي على المجلد الرئيسي لسطح المكتب `sumer-ui/` وكافة مجلدات البرامج النشطة بداخل `programs/` (مثل الشيل ودليل التلفاز الأندرويد)، ويقوم بتشغيل سكريبت التجميع المحلي لكل منها لإنتاج باينري متوافق مع Alpine، ثم ينسخ هذا الملف النهائي مباشرة إلى الجذر المباشر للمجلد الخاص بالتطبيق وبنفس اسمه (مثال: `sumer-ui/sumer-ui`، `programs/tigris-shell/tigris-shell`). هذا يسهل عليك رفعها كتحديثات مستقلة على جيت هاب!
    *   **الاستخدام:** `./scripts/build-all.sh`

---

### 2️⃣ `build-gui-rootfs.sh`
*   **English:**
    *   **Purpose:** The core OS assembler. It prepares the mini Alpine Linux RootFS, installs necessary system packages (udev, Slint dependencies, audio backends), copies the pre-compiled standalone binaries (`sumer-ui`, `android-tv-ui`, `tigris-shell`), creates the custom `/init` script for automatic boot and persistent OverlayFS loading, and bundles everything into the bootable `sumer-os.iso`.
    *   **Note:** It expects you to compile individual apps first using their local `./build.sh` script.
    *   **Usage:** Called automatically by `run.sh`.
*   **العربية:**
    *   **الوظيفة:** مهندس بناء النظام الأساسي. يقوم بتجهيز نظام التشغيل المصغر (Alpine Linux)، تثبيت التعريفات وملفات الصوت، ثم سحب البرامج المجمعة مسبقاً (`sumer-ui`, `android-tv-ui`, `tigris-shell`) ووضعها بالنظام، مع برمجة ملف الإقلاع الذكي الذي يقرأ الفلاشة ويحفظ ملفاتك تلقائياً (OverlayFS)، وينتهي بإنتاج ملف الـ ISO الإقلاعي.
    *   **ملاحظة:** يتطلب تجميع البرامج أولاً في مجلداتها عبر تشغيل `./build.sh` بداخلها.
    *   **الاستخدام:** يتم استدعاؤه تلقائياً بواسطة `run.sh`.

---

### 3️⃣ `run-app.sh`
*   **English:**
    *   **Purpose:** The rapid prototyping utility. Instead of spending minutes building the ISO and booting QEMU, this script runs the Rust application locally directly on your host Ubuntu/Debian system. It has been updated to dynamically run individual apps using command line arguments.
    *   **Usage:**
        *   `./scripts/run-app.sh` (Runs the main Desktop UI)
        *   `./scripts/run-app.sh tv` (Runs the Android TV UI independently)
        *   `./scripts/run-app.sh shell` (Runs the Tigris Shell CLI utility)
*   **العربية:**
    *   **الوظيفة:** أداة التطوير السريع. بدلاً من الانتظار لبناء النظام بالكامل وتشغيل المحاكي، تتيح لك هذه الأداة تشغيل واختبار الأكواد والبرامج والتعديلات الصوتية مباشرة وبشكل فوري على جهازك الحالي (Debian).
    *   **الاستخدام:**
        *   `./scripts/run-app.sh` (لتشغيل واجهة سطح المكتب الرئيسية)
        *   `./scripts/run-app.sh tv` (لتشغيل واجهة التلفاز المستقلة)
        *   `./scripts/run-app.sh shell` (لتشغيل شيل دجلة محلياً)

---

### 4️⃣ `launch.sh`
*   **English:**
    *   **Purpose:** The emulator runner. Boots the compiled `sumer-os.iso` in QEMU. It dynamically checks if a virtual USB drive image (`persist.img`) exists; if not, it automatically creates and formats it with an `ext4` filesystem and the label `SUMER-DATA` so you can debug the auto-save persistence features in QEMU.
    *   **Usage:** Called automatically by `run.sh` or used directly via `./scripts/launch.sh`.
*   **العربية:**
    *   **الوظيفة:** مشغل المحاكي الافتراضي. يقوم بتشغيل الـ ISO بداخل محاكي QEMU. ويقوم تلقائياً بفحص ما إذا كان هناك قرص حفظ بيانات وهمي (`persist.img`)؛ وإذا لم يكن موجوداً، يقوم بصناعته وفرمته تلقائياً بالاسم `SUMER-DATA` لتتمكن من تجربة ميزة الحفظ التلقائي في المحاكي.
    *   **الاستخدام:** يتم استدعاؤه تلقائياً بواسطة `run.sh` أو مباشرة عبر `./scripts/launch.sh`.

---

### 5️⃣ `clean.sh`
*   **English:**
    *   **Purpose:** Deep system clean utility. Deletes Alpine cache folders, Docker-compiled build directories (`target-alpine/`), local Rust debug builds (`target/`), and old ISO images. It prints the exact amount of disk space freed after cleaning.
    *   **Usage:** `./scripts/clean.sh`
*   **العربية:**
    *   **الوظيفة:** أداة تنظيف المساحة الشاملة. تقوم بحذف كافة الملفات المؤقتة، وملفات كاش Docker، وملفات التجميع المحلية المؤقتة (`target/`) وجميع ملفات الـ ISO القديمة، وتطبع لك المساحة المحررة بدقة متناهية.
    *   **الاستخدام:** `./scripts/clean.sh`

---

### 6️⃣ `preview.sh`
*   **English:**
    *   **Purpose:** Fast previewer. Launches QEMU directly using the existing `sumer-os.iso` without running any builder commands first. Ideal when you just want to reboot the system to test user persistence or bootloader configurations.
    *   **Usage:** `./scripts/preview.sh`
*   **العربية:**
    *   **الوظيفة:** العرض السريع والمباشر. يقوم بتشغيل محاكي QEMU فوراً باستخدام ملف الـ ISO المتوفر حالياً دون الدخول في عملية تجميع البرامج مجدداً. ممتاز لتجربة ثبات الحفظ التلقائي وإعدادات الإقلاع المتكررة.
    *   **الاستخدام:** `./scripts/preview.sh`

---

### 7️⃣ `folder2exe.sh`
*   **English:**
    *   **Purpose:** A template hook set up to later compile the **Rafed USB Flasher Utility** (a Windows `.exe` bundle that detects USB drives, formats a custom persistent partition labeled `SUMER-DATA`, and writes Sumer OS to it).
    *   **Usage:** Under construction / placeholder.
*   **العربية:**
    *   **الوظيفة:** ملف تحضيري مخصص للمستقبل، سيحتوي على أكواد بناء مجمع الفلاشات لنظام ويندوز (الذي يقوم بتهيئة الفلاشة وتقسيمها بالاسم `SUMER-DATA` وحرق النظام عليها).
    *   **الاستخدام:** قيد الإعداد للمراحل القادمة.
