# 📜 مفكرة ألواح (Alwah Notepad)

Bilingual documentation for the standalone note-taking application of Sumer OS.
وثائق ثنائية اللغة لتطبيق التدوين المستقل لنظام سومر.

---

## 🇸🇦 العربية

**مفكرة ألواح (Alwah Notepad)** هي تطبيق تدوين ونصوص متكامل ومستقل مصمم خصيصاً لنظام تشغيل سومر (Sumer OS)، مكتوب بلغة **Rust** السريعة والآمنة ومبني بواجهات **Slint** الرسومية المتطورة.

### ✨ المميزات:
1. **قائمة جانبية تفاعلية قابلة للإخفاء/الإظهار 📂:** يمكنك عرض الملاحظات أو إخفاء القائمة بضغطة زر واحدة من شريط الأدوات العلوي لتوفير مساحة كتابة واسعة.
2. **إدارة ملفات ذكية ومستقرة 💾:** يتم حفظ الملاحظات كملفات نصية خام (`.txt`) باسم الملاحظة مباشرة بداخل مجلد `/root/.notes/`. وبفضل طبقة حفظ النظام (OverlayFS) لن تُفقد ملاحظاتك أبداً عند إعادة التشغيل.
3. **بحث سريع 🔍:** إمكانية البحث الفوري بداخل قائمة الملاحظات للوصول للوح المطلوب.
4. **شريط حالة متطور 📊:** يعرض عدد الحروف والكلمات الحالية وحالة الحفظ الحالية بلون أخضر مريح.
5. **مظهر زجاجي شفاف فاخر (Dark Glassmorphism) 🎨:** مظهر جذاب يتماشى مع الهوية البصرية الراقية لنظام سومر.

### 🚀 التشغيل والتجميع:
* **التشغيل المحلي للتجربة والتعديل:**
  ```bash
  ./run.sh
  ```
* **البناء والتجميع لـ Sumer OS (Alpine Linux):**
  ```bash
  ./build.sh
  ```
  *(ينتج باينري مستقل متوافق بالكامل في المجلد الرئيسي لتطبيق ألواح).*

---

## 🇬🇧 English

**Alwah Notepad** is a premium, standalone text editor and note-taking application designed specifically for Sumer OS. It is engineered using **Rust** and rendered with **Slint UI**.

### ✨ Features:
1. **Collapsible Workspace Sidebar 📂:** Toggle the notes list sidebar on or off with a single toolbar click to maximize your writing area.
2. **Robust File Persistence 💾:** Notes are saved as raw `.txt` files named after their actual title under `/root/.notes/`. Thanks to the Sumer OS OverlayFS persistence layer, your writings are safely persisted across system reboots.
3. **Real-time Note Searching 🔍:** Instantly filter and locate notes by their titles.
4. **Status & Metrics Bar 📊:** View character and word counts, along with interactive green save status alerts.
5. **Dark Glassmorphic UI 🎨:** Curated deep visual aesthetics perfectly blending with Sumer OS workspace elements.

### 🚀 Build & Run:
* **Run locally for debug/development:**
  ```bash
  ./run.sh
  ```
* **Compile for Sumer OS (Alpine Linux target):**
  ```bash
  ./build.sh
  ```
  *(Produces a lightweight statically compiled binary ready to be bundled into the ISO).*
