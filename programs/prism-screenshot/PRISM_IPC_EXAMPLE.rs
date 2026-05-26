// 🔗 مثال: كيفية استخدام IPC من Prism-Screenshot

/*
هذا ملف مثال يوضح كيفية استخدام نظام IPC من داخل Prism-Screenshot
للتواصل مع Ur-Desktop والبرامج الأخرى.
*/

use std::fs;
use std::io::Write;

/// مثال 1: إخطار Ur-Desktop عند بدء التطبيق
pub fn notify_desktop_started() {
    let ipc_dir = "/tmp/sumer-os-ipc";
    
    // تأكد من وجود المجلد
    let _ = fs::create_dir_all(ipc_dir);
    
    // أرسل رسالة البدء
    let message = r#"{
        "app_id": "prism-screenshot",
        "status": "started",
        "timestamp": "2026-05-23T10:30:00Z",
        "message": "Prism Screenshot is ready"
    }"#;
    
    match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/prism-screenshot.msg", ipc_dir))
    {
        Ok(mut file) => {
            let _ = file.write_all(message.as_bytes());
            println!("✅ تم إرسال رسالة البدء إلى Ur-Desktop");
        },
        Err(e) => eprintln!("❌ خطأ: {}", e),
    }
}

/// مثال 2: إخطار عند التقاط لقطة شاشة
pub fn notify_screenshot_taken(file_path: &str) {
    let ipc_dir = "/tmp/sumer-os-ipc";
    
    // رسالة اللقطة
    let message = format!(r#"{{
        "app_id": "prism-screenshot",
        "event": "screenshot_taken",
        "file_path": "{}",
        "timestamp": "{}"
    }}"#, file_path, chrono::Local::now().to_rfc3339());
    
    match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/prism-screenshot.response", ipc_dir))
    {
        Ok(mut file) => {
            let _ = file.write_all(message.as_bytes());
            println!("✅ تم إخطار Ur-Desktop بلقطة جديدة: {}", file_path);
        },
        Err(e) => eprintln!("❌ خطأ: {}", e),
    }
}

/// مثال 3: قراءة الأوامر من Ur-Desktop
pub fn check_commands_from_desktop() -> Vec<String> {
    let ipc_dir = "/tmp/sumer-os-ipc";
    let cmd_file = format!("{}/prism-screenshot.cmd", ipc_dir);
    
    match fs::read_to_string(&cmd_file) {
        Ok(content) => {
            let commands: Vec<String> = content
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            println!("✅ استقبلت {} أوامر من Ur-Desktop", commands.len());
            
            // امسح الملف بعد القراءة
            let _ = fs::remove_file(&cmd_file);
            
            commands
        },
        Err(_) => {
            // لا توجد أوامر جديدة
            Vec::new()
        }
    }
}

/// مثال 4: إرسال حالة المعالجة
pub fn notify_processing(task: &str) {
    let ipc_dir = "/tmp/sumer-os-ipc";
    
    let message = format!(r#"{{
        "app_id": "prism-screenshot",
        "status": "processing",
        "task": "{}",
        "progress": 50
    }}"#, task);
    
    let _ = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}/prism-screenshot.status", ipc_dir))
        .and_then(|mut f| f.write_all(message.as_bytes()));
}

/// مثال 5: كود متكامل في main
pub fn main_with_ipc() {
    // عند البدء
    notify_desktop_started();
    
    // حلقة التطبيق الرئيسية
    loop {
        // تحقق من الأوامر الجديدة
        let commands = check_commands_from_desktop();
        
        for cmd in commands {
            println!("تنفيذ الأمر: {}", cmd);
            
            match cmd.as_str() {
                "take_full_screenshot" => {
                    notify_processing("taking full screenshot");
                    // ... كود التقاط اللقطة ...
                    notify_screenshot_taken("/tmp/screenshots/full_2026-05-23.png");
                },
                "take_region_screenshot" => {
                    notify_processing("taking region screenshot");
                    // ... كود التقاط اللقطة ...
                    notify_screenshot_taken("/tmp/screenshots/region_2026-05-23.png");
                },
                "exit" => {
                    println!("تم استقبال أمر الخروج من Ur-Desktop");
                    return;
                },
                _ => println!("أمر غير معروف: {}", cmd),
            }
        }
        
        // انتظر قليلاً قبل الفحص التالي
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

/// مثال 6: في prism-screenshot/src/main.rs
pub fn modify_prism_main() {
    /*
    // أضف هذا في بداية main():
    
    // إخطار الـ Desktop أن Prism بدأ التشغيل
    notify_desktop_started();
    
    // بعد التقاط لقطة، أرسل رسالة
    notify_screenshot_taken(&screenshot_path);
    
    // عند المعالجة
    notify_processing("applying filters");
    */
}

// 📝 استخدام متقدم: نظام Plugin

pub struct PluginSystem {
    plugins_dir: String,
}

impl PluginSystem {
    pub fn new() -> Self {
        PluginSystem {
            plugins_dir: "/tmp/sumer-os-plugins".to_string(),
        }
    }
    
    /// تشغيل plugin عند التقاط لقطة
    pub fn run_on_screenshot(&self, screenshot_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_dir = &self.plugins_dir;
        
        if !fs::metadata(plugin_dir).is_ok() {
            return Ok(());
        }
        
        // ابحث عن ملفات plugin
        if let Ok(entries) = fs::read_dir(plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "sh") == Some(true) {
                    // شغّل الـ script
                    println!("تشغيل plugin: {:?}", path);
                    let _ = std::process::Command::new(&path)
                        .arg(screenshot_path)
                        .spawn();
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ipc_directory_exists() {
        let ipc_dir = "/tmp/sumer-os-ipc";
        let _ = fs::create_dir_all(ipc_dir);
        assert!(fs::metadata(ipc_dir).is_ok());
    }
    
    #[test]
    fn test_notification_sending() {
        notify_desktop_started();
        // تحقق من أن الملف تم إنشاؤه
        assert!(fs::metadata("/tmp/sumer-os-ipc/prism-screenshot.msg").is_ok());
    }
}
