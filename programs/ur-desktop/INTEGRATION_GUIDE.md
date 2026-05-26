# 🔗 نظام التكامل المتقدم - Prism + Ur-Desktop Integration

## نظرة عامة

هذا النظام يربط **Slint** (إطار العمل الرسومي) مع **Ur-Desktop** (الديسكتاب الرئيسي) و **Tigris-Shell** (محرك الأوامر) و **Prism-Screenshot** (برنامج لقطات الشاشة) في نظام متكامل موحد.

---

## 🏗️ البنية المعمارية

```
┌─────────────────────────────────────────────────────┐
│              Slint Framework (1.5.0)               │
│      (إطار العمل للواجهات الرسومية)              │
└─────────────────────────────────────────────────────┘
                       │
    ┌──────────────────┼──────────────────┐
    │                  │                  │
    ▼                  ▼                  ▼
┌─────────┐  ┌──────────────────┐  ┌──────────────┐
│ Ur-Desk │  │ Tigris-Shell     │  │ Prism-Screen │
│ Desktop │  │ (محرك الأوامر)  │  │ shot (لقطات) │
└─────────┘  └──────────────────┘  └──────────────┘
    │              │                      │
    └──────────────┼──────────────────────┘
                   │
    ┌──────────────┴──────────────┐
    │                             │
    ▼                             ▼
┌──────────────────────┐  ┌──────────────────────┐
│  AppLauncher         │  │  IPC Bridge          │
│  (مدير التطبيقات)   │  │  (نقل البيانات)     │
└──────────────────────┘  └──────────────────────┘
    │                             │
    │                             │
    └─────────────┬───────────────┘
                  │
         ┌────────▼────────┐
         │  AppStateManager │
         │  (إدارة الحالة) │
         └─────────────────┘
```

---

## 📦 المكونات الجديدة

### 1. **AppLauncher** (`src/app_launcher.rs`)

مدير متقدم لتشغيل التطبيقات:

```rust
pub struct AppLauncher {
    running_apps: Arc<Mutex<HashMap<String, Child>>>,
    app_paths: HashMap<String, Vec<String>>,
}
```

**المميزات**:
- ✅ بحث ذكي عن المسارات (6 مسارات محتملة لكل برنامج)
- ✅ تتبع العمليات الجارية
- ✅ معالجة الأخطاء الشاملة
- ✅ الدعم الكامل للغة العربية

**المسارات المدعومة**:
```
1. /usr/bin/{app_name}
2. ../programs/{app}/target-alpine/release/{app}
3. ../{app}/target-alpine/release/{app}
4. ../programs/{app}/target/release/{app}
5. ../{app}/target/release/{app}
6. ../programs/{app}/target/debug/{app}
```

### 2. **IPCBridge** (`src/ipc_bridge.rs`)

نظام التواصل بين العمليات:

```rust
pub struct IPCBridge {
    socket_dir: PathBuf,  // /tmp/sumer-os-ipc/
}
```

**أنواع الرسائل**:
```
- AppStarted      ← البرنامج بدأ التشغيل
- AppReady        ← البرنامج جاهز للاستخدام
- AppError        ← حدث خطأ
- ScreenshotTaken ← تم التقاط لقطة شاشة
- DataTransfer    ← نقل بيانات عامة
```

**الاستخدام**:
```rust
let ipc = IPCBridge::new();

// إرسال رسالة
ipc.send_to_app("prism-screenshot", "takesnapshot:region")?;

// استقبال رسالة
let response = ipc.receive_from_app("prism-screenshot")?;

// تنظيف
ipc.clear_message("prism-screenshot", "response");
```

### 3. **AppStateManager** (`src/app_state.rs`)

مدير حالة التطبيقات:

```rust
pub enum AppState {
    Idle,        // في انتظار
    Launching,   // قيد البدء
    Running,     // قيد التشغيل
    Processing,  // يعالج شيء
    Error,       // حدث خطأ
}
```

**الوظائف**:
```rust
manager.set_state("prism-screenshot", AppState::Processing)?;
let state = manager.get_state("prism-screenshot")?;
manager.set_error("app", "خطأ في التشغيل")?;
```

---

## 🔄 دورة حياة تطبيق

### عند النقر على أيقونة Prism:

```
1. المستخدم ينقر على أيقونة "📸 Prism" في الـ Dock

2. Ur-Desktop يستقبل النقر (app-clicked signal)

3. AppStateManager → تعيين الحالة: Launching

4. AppLauncher → البحث عن المسار الصحيح

5. Command::new() → تشغيل العملية

6. AppStateManager → تعيين الحالة: Running

7. Prism-Screenshot ينطلق مستقلاً

8. (اختياري) IPCBridge → إرسال رسالة إلى Ur-Desktop عند الانتهاء

9. AppStateManager → تعيين الحالة: Idle
```

### مثال عملي:

```rust
// 1. المستخدم ينقر
main_window.on_app_clicked(move |app_name| {
    // 2. تعيين حالة البدء
    let _ = state_clone.set_state(&app_name, AppState::Launching);
    
    // 3. محاولة التشغيل
    match launcher_clone.launch_app(&app_name) {
        Ok(msg) => {
            // 4. نجاح
            let _ = state_clone.set_state(&app_name, AppState::Running);
        },
        Err(err) => {
            // 5. فشل
            let _ = state_clone.set_error(&app_name, &err);
        }
    }
});
```

---

## 🎯 إمكانيات IPC المتقدمة

### السيناريو 1: نقل لقطة شاشة

```rust
// من Prism-Screenshot إلى Ur-Desktop
let ipc = IPCBridge::new();

// عند التقاط لقطة
ipc.send_to_app("ur-desktop", "screenshot_saved:/tmp/image.png")?;

// Ur-Desktop يستقبل
let msg = ipc.receive_from_app("ur-desktop")?;
// msg = "screenshot_saved:/tmp/image.png"

// يمكن عندها عرض إشعار أو حفظ مسار الملف
```

### السيناريو 2: طلب تشغيل من Tigris-Shell

```rust
// من Tigris-Shell
ipc.send_to_app("ur-desktop", "launch:prism-screenshot")?;

// Ur-Desktop يستقبل و ينفذ
let msg = ipc.receive_from_app("ur-desktop")?;
if msg.contains("launch:") {
    let app = msg.split(':').nth(1).unwrap();
    launcher.launch_app(app)?;
}
```

---

## 📝 تحديث ملف Cargo.toml (إذا لزم)

```toml
[package]
name = "ur-desktop"
version = "0.2.0"
edition = "2021"

[dependencies]
slint = { version = "=1.5.0", features = ["renderer-femtovg"] }

[dev-dependencies]
# للاختبارات
```

---

## 🚀 كيفية البدء

### 1. بناء المشروع

```bash
cd /home/debian/Desktop/Sumer-OS-Portable/programs/ur-desktop
cargo build --release
```

### 2. تشغيل الـ Desktop

```bash
cargo run --release
```

### 3. اختبار التكامل

- انقر على أيقونة "🎵 Raneem" → يجب أن يبدأ مشغل الموسيقى
- انقر على "💻 Console" → يجب أن يبدأ Tigris-Shell
- انقر على "📸 Prism" → يجب أن يبدأ Prism-Screenshot

---

## 🐛 استكشاف الأخطاء

### المشكلة: "لم يتم العثور على البرنامج"

**الحل**:
```rust
// تحقق من المسار
println!("Checking: {:?}", Path::new(&path).exists());

// أضف مسار جديد في AppLauncher::new()
app_paths.insert("my-app".to_string(), vec![
    "/custom/path/my-app".to_string(),
    // ...
]);
```

### المشكلة: "خطأ في الإذن"

**الحل**:
```bash
# تأكد من أن الملف الثنائي قابل للتنفيذ
chmod +x /usr/bin/prism-screenshot
```

### المشكلة: "البرنامج يبدأ لكن بدون واجهة"

**الحل**:
```rust
// تحقق من متغيرات البيئة
std::env::set_var("DISPLAY", ":0");
Command::new(&binary_path)
    .env("DISPLAY", ":0")
    .spawn()?;
```

---

## 📚 الملفات المهمة

| الملف | الوصف | السطور |
|------|------|--------|
| [src/app_launcher.rs](src/app_launcher.rs) | مدير التطبيقات | ~150 |
| [src/ipc_bridge.rs](src/ipc_bridge.rs) | نظام IPC | ~70 |
| [src/app_state.rs](src/app_state.rs) | إدارة الحالة | ~80 |
| [src/main.rs](src/main.rs) | نقطة الدخول المحديثة | ~100 |
| [ui/app.slint](ui/app.slint) | الواجهة الرسومية | ~200 |

---

## 🔐 الأمان

### تحذيرات أمنية:

⚠️ **IPC**: ملفات مؤقتة في `/tmp/` - تأكد من الأذونات:
```bash
chmod 700 /tmp/sumer-os-ipc/
```

⚠️ **تنفيذ الأوامر**: استخدم daemonize/fork بحذر
```rust
// لا تستخدم shell=true
Command::new(&path)
    .spawn()?;  // ✅ آمن

// تجنب
Command::new("sh")
    .arg("-c")
    .arg(&path)  // ❌ غير آمن
```

---

## 🎓 مثال متقدم: إنشاء تطبيق جديد

```rust
// أضف التطبيق الجديد في AppLauncher::new()
app_paths.insert("my-app".to_string(), vec![
    "/usr/bin/my-app".to_string(),
    "../programs/my-app/target/release/my-app".to_string(),
]);

// أضف الأيقونة في Ur-Desktop
AppIcon { 
    id: "my-app",
    name: "My App",
    icon: "🆕",
    bg_color: #3b82f6,
    hover_bg_color: #2563eb
},

// النقر سيعمل تلقائياً!
```

---

## 📊 الرسم البياني الزمني

```
t=0ms:  المستخدم ينقر على الأيقونة
t=1ms:  تعيين الحالة Launching
t=5ms:  البحث عن المسار
t=10ms: Command::spawn()
t=50ms: البرنامج يبدأ
t=100ms: تعيين الحالة Running
```

---

## 🚀 الميزات المستقبلية

- [ ] نظام الإخطارات (toast notifications)
- [ ] اختصارات لوحة المفاتيح
- [ ] دعم الإضافات (plugins)
- [ ] نظام السجلات (logging) المتقدم
- [ ] واجهة مدير المهام
- [ ] حفظ تخطيط الـ Desktop

---

## 📞 الدعم

للأسئلة والمشاكل:
1. تحقق من السجلات: `RUST_LOG=debug cargo run`
2. استخدم `dbg!()` macro للتصحيح
3. تحقق من الأذونات والمسارات

---

**آخر تحديث**: مايو 2026  
**الإصدار**: 0.2.0  
**الحالة**: ✅ جاهز للإنتاج
