// 🔗 Prism-Screenshot Integration Module
// وحدة الربط - تربط Prism مع Ur-Desktop عبر البرنامج الثنائي

use std::path::PathBuf;
use std::fs;
use std::process::{Command, Stdio};

pub mod binary_launcher;

/// 🎯 نقطة تكامل Prism مع النظام الأكبر
pub struct PrismIntegration {
    binary_path: PathBuf,
    ipc_dir: PathBuf,
}

impl PrismIntegration {
    /// إنشاء نقطة تكامل جديدة
    pub fn new() -> Result<Self, String> {
        let binary_path = PathBuf::from("target/release/prism-screenshot")
            .canonicalize()
            .or_else(|_| PathBuf::from("target-alpine/release/prism-screenshot").canonicalize())
            .or_else(|_| Ok(PathBuf::from("/usr/bin/prism-screenshot")))
            .unwrap_or_else(|_| PathBuf::from("prism-screenshot"));
        
        let ipc_dir = PathBuf::from("/tmp/sumer-os-ipc");
        
        // أنشئ مجلد IPC إذا لم يكن موجوداً
        let _ = fs::create_dir_all(&ipc_dir);
        
        Ok(PrismIntegration {
            binary_path,
            ipc_dir,
        })
    }
    
    /// ابدأ تشغيل Prism من Ur-Desktop
    pub fn launch_from_desktop(&self) -> Result<String, String> {
        println!("🚀 جاري تشغيل Prism-Screenshot من Ur-Desktop...");
        
        // أرسل رسالة بدء إلى Ur-Desktop
        self.notify_desktop_starting()?;
        
        // شغّل البرنامج
        let child = Command::new(&self.binary_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("❌ خطأ في التشغيل: {}", e))?;
        
        println!("✅ تم تشغيل Prism-Screenshot بنجاح (PID: {})", child.id());
        
        // أرسل رسالة البدء الناجح
        self.notify_desktop_started()?;
        
        Ok(format!("✅ تم تشغيل Prism-Screenshot"))
    }
    
    /// أرسل إخطار بدء للـ Desktop
    fn notify_desktop_starting(&self) -> Result<(), String> {
        let msg = r#"{"app": "prism-screenshot", "event": "starting", "timestamp": "now"}"#;
        
        let msg_file = self.ipc_dir.join("prism-screenshot.starting");
        fs::write(&msg_file, msg)
            .map_err(|e| format!("خطأ: {}", e))?;
        
        println!("📤 تم إرسال إخطار البدء");
        Ok(())
    }
    
    /// أرسل إخطار البدء الناجح للـ Desktop
    fn notify_desktop_started(&self) -> Result<(), String> {
        let msg = r#"{"app": "prism-screenshot", "event": "started", "status": "ready"}"#;
        
        let msg_file = self.ipc_dir.join("prism-screenshot.started");
        fs::write(&msg_file, msg)
            .map_err(|e| format!("خطأ: {}", e))?;
        
        println!("📤 تم إرسال إخطار البدء الناجح");
        Ok(())
    }
    
    /// احصل على مسار الـ Binary
    pub fn get_binary_path(&self) -> &PathBuf {
        &self.binary_path
    }
    
    /// احصل على مسار IPC
    pub fn get_ipc_dir(&self) -> &PathBuf {
        &self.ipc_dir
    }
}

/// قاموس تشغيل البرامج - App Launcher Dictionary
pub mod binary_launcher {
    use std::path::PathBuf;
    use std::process::Command;
    
    pub struct AppLauncher {
        apps: std::collections::HashMap<String, PathBuf>,
    }
    
    impl AppLauncher {
        pub fn new() -> Self {
            let mut apps = std::collections::HashMap::new();
            
            // أضف Prism-Screenshot
            apps.insert(
                "prism-screenshot".to_string(),
                PathBuf::from("target/release/prism-screenshot"),
            );
            
            // يمكن إضافة تطبيقات أخرى هنا
            
            AppLauncher { apps }
        }
        
        pub fn launch(&self, app_name: &str) -> Result<String, String> {
            let path = self.apps.get(app_name)
                .ok_or_else(|| format!("التطبيق '{}' غير موجود", app_name))?;
            
            println!("🚀 جاري تشغيل: {}", app_name);
            
            Command::new(path)
                .spawn()
                .map_err(|e| format!("❌ خطأ: {}", e))?;
            
            Ok(format!("✅ تم تشغيل {}", app_name))
        }
        
        pub fn register_app(&mut self, name: String, path: PathBuf) {
            self.apps.insert(name, path);
            println!("✅ تم تسجيل التطبيق");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prism_integration_creation() {
        match PrismIntegration::new() {
            Ok(integration) => {
                println!("✅ تم إنشاء التكامل بنجاح");
                println!("   Binary: {:?}", integration.get_binary_path());
                println!("   IPC Dir: {:?}", integration.get_ipc_dir());
            },
            Err(e) => eprintln!("❌ {}", e),
        }
    }
    
    #[test]
    fn test_app_launcher() {
        let launcher = binary_launcher::AppLauncher::new();
        println!("✅ تم إنشاء App Launcher بنجاح");
    }
}
