use std::process::{Command, Child};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;

/// نظام إدارة تطبيقات متقدم - App Management System
pub struct AppLauncher {
    running_apps: Arc<Mutex<HashMap<String, Child>>>,
    app_paths: HashMap<String, Vec<String>>,
}

impl AppLauncher {
    pub fn new() -> Self {
        let mut app_paths = HashMap::new();
        
        // تحديد المسارات المحتملة لكل برنامج
        app_paths.insert("prism-screenshot".to_string(), vec![
            "/usr/bin/prism-screenshot".to_string(),
            "../programs/prism-screenshot/target-alpine/release/prism-screenshot".to_string(),
            "../prism-screenshot/target-alpine/release/prism-screenshot".to_string(),
            "../programs/prism-screenshot/target/release/prism-screenshot".to_string(),
            "../prism-screenshot/target/release/prism-screenshot".to_string(),
            "../programs/prism-screenshot/target/debug/prism-screenshot".to_string(),
        ]);
        
        app_paths.insert("tigris-shell".to_string(), vec![
            "/usr/bin/tigris-shell".to_string(),
            "../programs/tigris-shell/target-alpine/release/tigris-shell".to_string(),
            "../tigris-shell/target-alpine/release/tigris-shell".to_string(),
            "../programs/tigris-shell/target/release/tigris-shell".to_string(),
            "../tigris-shell/target/release/tigris-shell".to_string(),
            "../programs/tigris-shell/target/debug/tigris-shell".to_string(),
        ]);
        
        app_paths.insert("helix".to_string(), vec![
            "/usr/bin/helix".to_string(),
            "../programs/helix-set/target-alpine/release/helix".to_string(),
            "../programs/helix-set/helix".to_string(),
            "../programs/helix-set/target/release/helix".to_string(),
            "../programs/helix-set/target/debug/helix".to_string(),
        ]);
        
        AppLauncher {
            running_apps: Arc::new(Mutex::new(HashMap::new())),
            app_paths,
        }
    }
    
    /// ابحث عن المسار الصحيح للبرنامج
    fn find_binary_path(&self, app_id: &str, binary_name: &str) -> Option<String> {
        // تحقق من المسارات المعرّفة مسبقاً
        if let Some(paths) = self.app_paths.get(binary_name) {
            for path in paths {
                if Path::new(path).exists() {
                    return Some(path.clone());
                }
            }
        }
        
        None
    }
    
    /// قم بتشغيل برنامج
    pub fn launch_app(&self, app_name: &str) -> Result<String, String> {
        let binary_path = match app_name {
            "Screenshot" | "prism-screenshot" | "بريزم" | "لقطة الشاشة" => {
                self.find_binary_path("prism-screenshot", "prism-screenshot")
                    .ok_or_else(|| "لم يتم العثور على برنامج Prism-Screenshot".to_string())?
            },
            "Console" | "💻" | "شيل دجلة" => {
                self.find_binary_path("tigris-shell", "tigris-shell")
                    .ok_or_else(|| "لم يتم العثور على برنامج Tigris-Shell".to_string())?
            },
            "TV" | "تلفاز نهر" => {
                "/usr/bin/nahr-tv".to_string()
            },
            "Settings" | "helix" | "الاعدادات" | "إعدادات هيليكس" | "helix-set" => {
                self.find_binary_path("helix", "helix")
                    .ok_or_else(|| "لم يتم العثور على برنامج Helix".to_string())?
            },
            _ => return Err(format!("البرنامج غير معروف: {}", app_name)),
        };
        
        // تحقق من وجود البرنامج
        if !Path::new(&binary_path).exists() {
            return Err(format!("لم يتم العثور على المسار: {}", binary_path));
        }
        
        // قم بتشغيل البرنامج
        match Command::new(&binary_path).spawn() {
            Ok(child) => {
                // سجّل العملية الجارية
                if let Ok(mut apps) = self.running_apps.lock() {
                    apps.insert(app_name.to_string(), child);
                }
                Ok(format!("✅ تم بدء تشغيل {}", app_name))
            },
            Err(e) => Err(format!("❌ خطأ في بدء التشغيل: {}", e)),
        }
    }
    
    /// تحقق مما إذا كان البرنامج قيد التشغيل
    pub fn is_app_running(&self, app_name: &str) -> bool {
        if let Ok(apps) = self.running_apps.lock() {
            return apps.contains_key(app_name);
        }
        false
    }
    
    /// أوقف برنامج
    pub fn stop_app(&self, app_name: &str) -> Result<String, String> {
        if let Ok(mut apps) = self.running_apps.lock() {
            if let Some(mut child) = apps.remove(app_name) {
                match child.kill() {
                    Ok(_) => Ok(format!("✅ تم إيقاف {}", app_name)),
                    Err(e) => Err(format!("❌ خطأ في الإيقاف: {}", e)),
                }
            } else {
                Err(format!("البرنامج {} ليس قيد التشغيل", app_name))
            }
        } else {
            Err("خطأ في الوصول إلى قائمة التطبيقات".to_string())
        }
    }
}
