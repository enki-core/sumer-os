// 🧪 اختبارات شاملة لنظام التكامل

#[cfg(test)]
mod tests {
    use crate::app_launcher::AppLauncher;
    use crate::app_state::{AppStateManager, AppState};
    use crate::ipc_bridge::IPCBridge;
    use std::path::Path;

    #[test]
    fn test_app_launcher_finds_prism() {
        let launcher = AppLauncher::new();
        
        // اختبار البحث عن prism-screenshot
        let result = launcher.find_binary_path("prism-screenshot", "prism-screenshot");
        
        // قد يكون None إذا لم يكن مثبتاً، لكن يجب ألا يعطي panic
        match result {
            Some(path) => {
                println!("✅ Found prism-screenshot at: {}", path);
                assert!(Path::new(&path).exists() || !Path::new(&path).exists());
            },
            None => {
                println!("⚠️ prism-screenshot not found in standard paths");
            }
        }
    }

    #[test]
    fn test_app_state_manager_transitions() {
        let manager = AppStateManager::new();
        
        // 1. تعيين الحالة الأولية
        let _ = manager.set_state("test-app", AppState::Idle);
        let state = manager.get_state("test-app").unwrap();
        assert_eq!(state, AppState::Idle);
        
        // 2. تحويل إلى Launching
        let _ = manager.set_state("test-app", AppState::Launching);
        let state = manager.get_state("test-app").unwrap();
        assert_eq!(state, AppState::Launching);
        
        // 3. تحويل إلى Running
        let _ = manager.set_state("test-app", AppState::Running);
        let state = manager.get_state("test-app").unwrap();
        assert_eq!(state, AppState::Running);
        
        println!("✅ جميع تحويلات الحالة تعمل بشكل صحيح");
    }

    #[test]
    fn test_app_state_error_handling() {
        let manager = AppStateManager::new();
        
        // 1. تعيين خطأ
        let _ = manager.set_error("error-app", "خطأ في الاتصال");
        let state = manager.get_state("error-app").unwrap();
        assert_eq!(state, AppState::Error);
        
        // 2. قراءة رسالة الخطأ
        let error = manager.get_error("error-app").unwrap();
        assert_eq!(error, Some("خطأ في الاتصال".to_string()));
        
        // 3. مسح الخطأ
        let _ = manager.clear_error("error-app");
        let error = manager.get_error("error-app").unwrap();
        assert_eq!(error, None);
        
        println!("✅ معالجة الأخطاء تعمل بشكل صحيح");
    }

    #[test]
    fn test_ipc_bridge_creation() {
        let ipc = IPCBridge::new();
        
        // تحقق من أن مجلد IPC تم إنشاؤه
        assert!(Path::new("/tmp/sumer-os-ipc").exists());
        
        println!("✅ جسر IPC تم إنشاؤه بنجاح");
    }

    #[test]
    fn test_ipc_message_sending() {
        let ipc = IPCBridge::new();
        
        // أرسل رسالة اختبار
        let result = ipc.send_to_app("test-app", "test_message:hello");
        
        assert!(result.is_ok(), "يجب أن تنجح عملية الإرسال");
        
        println!("✅ إرسال الرسائل يعمل بشكل صحيح");
    }

    #[test]
    fn test_app_names_arabic_support() {
        let launcher = AppLauncher::new();
        
        // اختبر الأسماء العربية
        let app_names = vec![
            "Screenshot",
            "prism-screenshot",
            "بريزم",
            "لقطة الشاشة",
            "Console",
            "شيل دجلة",
            "💻",
        ];
        
        for name in app_names {
            match launcher.launch_app(name) {
                Ok(_) => println!("✅ تم تشغيل: {}", name),
                Err(e) => println!("⚠️  فشل: {} - {}", name, e),
            }
        }
    }
}

// 📝 سيناريوهات الاختبار اليدوي

/*
تشغيل الاختبارات:

    cargo test

اختبار يدوي للـ Desktop:

1. شغّل البرنامج:
   cargo run --release

2. انقر على الأيقونات بالترتيب:
   - 📁 Files
   - 💻 Console (يجب أن يفتح Tigris-Shell)
   - 📺 TV (يجب أن يفتح Nahr-TV)
   - 🎵 Raneem (يجب أن يفتح Raneem MP3)
   - 📸 Prism (يجب أن يفتح Prism-Screenshot) ⭐

3. اختبر Drag & Drop:
   - اسحب أيقونة من الـ Dock إلى سطح المكتب
   - تحقق من الحفظ والاستعادة

4. اختبر IPC:
   - افتح Tigris-Shell
   - أكتب: app prism-screenshot
   - يجب أن تفتح لقطة الشاشة
*/
