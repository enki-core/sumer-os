use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// حالة التطبيق - Application State
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Idle,        // في انتظار الاستخدام
    Launching,   // قيد البدء
    Running,     // قيد التشغيل
    Processing,  // يعالج شيء ما (مثل التقاط لقطة)
    Error,       // حدث خطأ
}

pub struct AppStateManager {
    states: Arc<Mutex<HashMap<String, AppState>>>,
    errors: Arc<Mutex<HashMap<String, String>>>,
}

impl AppStateManager {
    pub fn new() -> Self {
        AppStateManager {
            states: Arc::new(Mutex::new(HashMap::new())),
            errors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// عيّن حالة التطبيق
    pub fn set_state(&self, app_id: &str, state: AppState) -> Result<(), String> {
        if let Ok(mut states) = self.states.lock() {
            states.insert(app_id.to_string(), state);
            Ok(())
        } else {
            Err("خطأ في الوصول إلى قائمة الحالات".to_string())
        }
    }
    
    /// احصل على حالة التطبيق
    pub fn get_state(&self, app_id: &str) -> Result<AppState, String> {
        if let Ok(states) = self.states.lock() {
            Ok(states.get(app_id).copied().unwrap_or(AppState::Idle))
        } else {
            Err("خطأ في الوصول إلى قائمة الحالات".to_string())
        }
    }
    
    /// سجّل خطأ
    pub fn set_error(&self, app_id: &str, error_msg: &str) -> Result<(), String> {
        if let Ok(mut errors) = self.errors.lock() {
            errors.insert(app_id.to_string(), error_msg.to_string());
            self.set_state(app_id, AppState::Error)?;
            Ok(())
        } else {
            Err("خطأ في الوصول إلى سجل الأخطاء".to_string())
        }
    }
    
    /// احصل على رسالة الخطأ
    pub fn get_error(&self, app_id: &str) -> Result<Option<String>, String> {
        if let Ok(errors) = self.errors.lock() {
            Ok(errors.get(app_id).cloned())
        } else {
            Err("خطأ في الوصول إلى سجل الأخطاء".to_string())
        }
    }
    
    /// امسح الخطأ
    pub fn clear_error(&self, app_id: &str) -> Result<(), String> {
        if let Ok(mut errors) = self.errors.lock() {
            errors.remove(app_id);
            Ok(())
        } else {
            Err("خطأ في الوصول إلى سجل الأخطاء".to_string())
        }
    }
}
