use std::fs;
use std::path::PathBuf;
use std::io::{Write, Read};

/// نظام التواصل بين العمليات - IPC Bridge
/// يسمح لـ prism-screenshot بإرسال البيانات والحالات إلى ur-desktop

#[derive(Debug, Clone)]
pub struct IPCMessage {
    pub app_id: String,
    pub message_type: MessageType,
    pub data: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    AppStarted,      // البرنامج بدأ التشغيل
    AppReady,        // البرنامج جاهز
    AppError,        // حدث خطأ
    ScreenshotTaken, // تم التقاط لقطة شاشة
    DataTransfer,    // نقل بيانات
}

pub struct IPCBridge {
    socket_dir: PathBuf,
}

impl IPCBridge {
    pub fn new() -> Self {
        let socket_dir = PathBuf::from("/tmp/sumer-os-ipc");
        let _ = fs::create_dir_all(&socket_dir);
        
        IPCBridge { socket_dir }
    }
    
    /// أرسل رسالة من ur-desktop إلى تطبيق
    pub fn send_to_app(&self, app_id: &str, message: &str) -> Result<(), String> {
        let file_path = self.socket_dir.join(format!("{}.msg", app_id));
        
        match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file_path)
        {
            Ok(mut file) => {
                file.write_all(message.as_bytes())
                    .map_err(|e| format!("خطأ في الكتابة: {}", e))?;
                Ok(())
            },
            Err(e) => Err(format!("خطأ في فتح الملف: {}", e)),
        }
    }
    
    /// اقرأ رسالة من تطبيق
    pub fn receive_from_app(&self, app_id: &str) -> Result<String, String> {
        let file_path = self.socket_dir.join(format!("{}.response", app_id));
        
        match fs::OpenOptions::new().read(true).open(&file_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| format!("خطأ في القراءة: {}", e))?;
                Ok(contents)
            },
            Err(e) => Err(format!("لا توجد رسالة: {}", e)),
        }
    }
    
    /// أزل ملف الرسالة بعد المعالجة
    pub fn clear_message(&self, app_id: &str, msg_type: &str) {
        let _ = fs::remove_file(self.socket_dir.join(format!("{}.{}", app_id, msg_type)));
    }
}
