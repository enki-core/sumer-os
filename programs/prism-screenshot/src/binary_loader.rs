// 🔗 Prism-Screenshot Binary Loader
// قارئ الملف الثنائي - برنامج قراءة الـ Binary من مجلد Prism

use std::path::{Path, PathBuf};
use std::fs;
use std::process::{Command, Child};
use std::env;

/// 🎯 محمل الملف الثنائي - Binary Loader
pub struct BinaryLoader {
    binary_path: PathBuf,
    app_name: String,
}

impl BinaryLoader {
    /// إنشء محمل جديد
    pub fn new() -> Result<Self, String> {
        // الحصول على مجلد المشروع الحالي
        let current_dir = env::current_dir()
            .map_err(|e| format!("❌ خطأ في الحصول على المجلد الحالي: {}", e))?;
        
        println!("📁 المجلد الحالي: {:?}", current_dir);
        
        // البحث عن الملف الثنائي في عدة مسارات
        let possible_paths = vec![
            PathBuf::from("target/release/prism-screenshot"),
            PathBuf::from("target-alpine/release/prism-screenshot"),
            PathBuf::from("target/debug/prism-screenshot"),
            PathBuf::from("../prism-screenshot/target/release/prism-screenshot"),
            PathBuf::from("/usr/bin/prism-screenshot"),
        ];
        
        let mut found_path = None;
        
        for path in &possible_paths {
            let full_path = if path.is_absolute() {
                path.clone()
            } else {
                current_dir.join(path)
            };
            
            if full_path.exists() {
                println!("✅ وُجد الملف الثنائي: {:?}", full_path);
                found_path = Some(full_path);
                break;
            }
        }
        
        let binary_path = found_path
            .ok_or_else(|| {
                format!("❌ لم يتم العثور على الملف الثنائي في المسارات:\n{:?}", possible_paths)
            })?;
        
        Ok(BinaryLoader {
            binary_path,
            app_name: "prism-screenshot".to_string(),
        })
    }
    
    /// احصل على معلومات الملف الثنائي
    pub fn get_binary_info(&self) -> Result<BinaryInfo, String> {
        let metadata = fs::metadata(&self.binary_path)
            .map_err(|e| format!("❌ خطأ في قراءة معلومات الملف: {}", e))?;
        
        Ok(BinaryInfo {
            path: self.binary_path.clone(),
            size: metadata.len(),
            is_executable: Self::is_executable(&self.binary_path),
            app_name: self.app_name.clone(),
        })
    }
    
    /// تحقق من أن الملف قابل للتنفيذ
    fn is_executable(path: &Path) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(path) {
                let permissions = metadata.permissions();
                return permissions.mode() & 0o111 != 0;
            }
        }
        true
    }
    
    /// شغّل الملف الثنائي
    pub fn launch(&self) -> Result<Child, String> {
        println!("🚀 جاري تشغيل: {:?}", self.binary_path);
        
        Command::new(&self.binary_path)
            .spawn()
            .map_err(|e| format!("❌ خطأ في تشغيل البرنامج: {}", e))
    }
    
    /// تحقق من أن الملف الثنائي صالح
    pub fn validate(&self) -> Result<(), String> {
        // تحقق من الوجود
        if !self.binary_path.exists() {
            return Err("❌ الملف الثنائي غير موجود".to_string());
        }
        
        // تحقق من أنه ملف وليس مجلد
        let metadata = fs::metadata(&self.binary_path)
            .map_err(|e| format!("❌ خطأ في قراءة الملف: {}", e))?;
        
        if !metadata.is_file() {
            return Err("❌ البرنامج ليس ملفاً عادياً".to_string());
        }
        
        // تحقق من أنه قابل للتنفيذ
        if !Self::is_executable(&self.binary_path) {
            return Err("⚠️ تحذير: الملف قد لا يكون قابلاً للتنفيذ".to_string());
        }
        
        println!("✅ الملف الثنائي صحيح وجاهز للتشغيل");
        Ok(())
    }
    
    /// احصل على مسار الملف الثنائي
    pub fn get_path(&self) -> &Path {
        &self.binary_path
    }
    
    /// احصل على اسم التطبيق
    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }
}

/// معلومات الملف الثنائي
#[derive(Debug, Clone)]
pub struct BinaryInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_executable: bool,
    pub app_name: String,
}

impl BinaryInfo {
    /// طباعة المعلومات
    pub fn print(&self) {
        println!("\n📊 معلومات البرنامج:");
        println!("  📝 الاسم: {}", self.app_name);
        println!("  📁 المسار: {:?}", self.path);
        println!("  💾 الحجم: {} بايت ({:.2} MB)", 
            self.size,
            self.size as f64 / (1024.0 * 1024.0)
        );
        println!("  ✅ قابل للتنفيذ: {}", if self.is_executable { "نعم" } else { "لا" });
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_binary_loader_creation() {
        match BinaryLoader::new() {
            Ok(loader) => {
                println!("✅ تم إنشاء المحمل بنجاح");
                println!("   التطبيق: {}", loader.get_app_name());
                println!("   المسار: {:?}", loader.get_path());
            },
            Err(e) => println!("⚠️ تحذير: {}", e),
        }
    }
    
    #[test]
    fn test_binary_validation() {
        if let Ok(loader) = BinaryLoader::new() {
            match loader.validate() {
                Ok(_) => println!("✅ التحقق من الملف نجح"),
                Err(e) => println!("⚠️ التحقق: {}", e),
            }
        }
    }
    
    #[test]
    fn test_binary_info() {
        if let Ok(loader) = BinaryLoader::new() {
            if let Ok(info) = loader.get_binary_info() {
                info.print();
            }
        }
    }
}
