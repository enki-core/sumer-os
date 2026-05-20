use std::sync::{Arc, Mutex};
use slint::Model;
use crate::models::{AppWindow, AppInfo};
use crate::store::refresh_apps_list;
use crate::launcher::launch_application;
use std::rc::Rc;

pub fn handle_tab_changed(app_weak: slint::Weak<AppWindow>, all_apps: Arc<Mutex<Vec<AppInfo>>>, idx: i32) {
    if let Some(app) = app_weak.upgrade() {
        app.set_active_tab(idx);
        refresh_apps_list(&app, &all_apps.lock().unwrap());
    }
}

pub fn handle_search_changed(app_weak: slint::Weak<AppWindow>, all_apps: Arc<Mutex<Vec<AppInfo>>>, query: slint::SharedString) {
    if let Some(app) = app_weak.upgrade() {
        app.set_search_query(query);
        refresh_apps_list(&app, &all_apps.lock().unwrap());
    }
}

pub fn handle_launch_app(app_weak: slint::Weak<AppWindow>, app_id: slint::SharedString) {
    if let Some(app) = app_weak.upgrade() {
        let app_id_str = app_id.to_string();
        app.set_status_msg(format!("جاري إقلاع وتشغيل التطبيق: {}... 🚀", app_id_str).into());
        
        // Spawn app in background thread
        std::thread::spawn(move || {
            launch_application(&app_id_str);
        });
    }
}

pub fn handle_install_app(app_weak: slint::Weak<AppWindow>, all_apps: Arc<Mutex<Vec<AppInfo>>>, idx: i32) {
    if let Some(app) = app_weak.upgrade() {
        let apps_model = app.get_apps();
        if let Some(app_info) = apps_model.row_data(idx as usize) {
            app.set_status_msg(format!("جاري تنزيل {} وتأمين ملفاته... 📥⏳", app_info.name).into());
            
            let app_weak_inner = app_weak.clone();
            let all_apps_inner = all_apps.clone();
            let app_id = app_info.id.to_string();
            let app_name = app_info.name.to_string();
            
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(2500));
                
                let all_apps_inner2 = all_apps_inner.clone();
                let app_id_clone = app_id.clone();
                let app_name_clone = app_name.clone();
                let app_weak_inner2 = app_weak_inner.clone();
                
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak_inner2.upgrade() {
                        {
                            let mut master = all_apps_inner2.lock().unwrap();
                            if let Some(a) = master.iter_mut().find(|a| a.id == app_id_clone) {
                                a.status = "installed".into();
                            }
                        }
                        refresh_apps_list(&app, &all_apps_inner2.lock().unwrap());
                        app.set_status_msg(format!("🎉 تم تثبيت {} بنجاح! جاهز للتشغيل الآن.", app_name_clone).into());
                    }
                });
            });
        }
    }
}

pub fn handle_update_app(app_weak: slint::Weak<AppWindow>, all_apps: Arc<Mutex<Vec<AppInfo>>>, idx: i32) {
    if let Some(app) = app_weak.upgrade() {
        let apps_model = app.get_apps();
        if let Some(app_info) = apps_model.row_data(idx as usize) {
            app.set_status_msg(format!("جاري ترقية حزمة {} وإعادة تجميع التحديث... 🔄⏳", app_info.name).into());
            
            let app_weak_inner = app_weak.clone();
            let all_apps_inner = all_apps.clone();
            let app_id = app_info.id.to_string();
            let app_name = app_info.name.to_string();
            
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(2500));
                
                let all_apps_inner2 = all_apps_inner.clone();
                let app_id_clone = app_id.clone();
                let app_name_clone = app_name.clone();
                let app_weak_inner2 = app_weak_inner.clone();
                
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak_inner2.upgrade() {
                        {
                            let mut master = all_apps_inner2.lock().unwrap();
                            if let Some(a) = master.iter_mut().find(|a| a.id == app_id_clone) {
                                a.status = "installed".into();
                                let old_version = a.version.to_string();
                                a.version = format!("{}.1", old_version).into();
                            }
                        }
                        refresh_apps_list(&app, &all_apps_inner2.lock().unwrap());
                        app.set_status_msg(format!("🎉 تم تحديث وترقية {} إلى أحدث إصدار بنجاح! 🟢", app_name_clone).into());
                    }
                });
            });
        }
    }
}

pub fn handle_update_all_apps(app_weak: slint::Weak<AppWindow>, all_apps: Arc<Mutex<Vec<AppInfo>>>) {
    if let Some(app) = app_weak.upgrade() {
        app.set_status_msg("جاري فحص وتحديث كل حزم التطبيقات المعلقة دفعة واحدة... 🔄📦".into());
        
        let app_weak_inner = app_weak.clone();
        let all_apps_inner = all_apps.clone();
        
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(3000));
            
            let all_apps_inner2 = all_apps_inner.clone();
            let app_weak_inner2 = app_weak_inner.clone();
            
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = app_weak_inner2.upgrade() {
                    let mut updated_count = 0;
                    {
                        let mut master = all_apps_inner2.lock().unwrap();
                        for a in master.iter_mut() {
                            if a.status == "update_available" {
                                a.status = "installed".into();
                                let old_version = a.version.to_string();
                                a.version = format!("{}.1", old_version).into();
                                updated_count += 1;
                            }
                        }
                    }
                    refresh_apps_list(&app, &all_apps_inner2.lock().unwrap());
                    app.set_status_msg(format!("🎉 تم ترقية وتحديث جميع التطبيقات الـ ({}) بنجاح تام! 📦🟢", updated_count).into());
                }
            });
        });
    }
}

pub fn handle_toggle_addon(app_weak: slint::Weak<AppWindow>, idx: i32) {
    if let Some(app) = app_weak.upgrade() {
        let addons_model = app.get_addons();
        let mut new_addons = Vec::new();
        let mut status_str = String::new();
        for i in 0..addons_model.row_count() {
            if let Some(mut addon) = addons_model.row_data(i) {
                if i == idx as usize {
                    addon.enabled = !addon.enabled;
                    if addon.enabled {
                        status_str = format!("🟢 تم تفعيل الإضافة: {} بنجاح حياً!", addon.name);
                    } else {
                        status_str = format!("🔴 تم تعطيل الإضافة: {} مؤقتاً.", addon.name);
                    }
                }
                new_addons.push(addon);
            }
        }
        let new_model = Rc::new(slint::VecModel::from(new_addons));
        app.set_addons(new_model.into());
        app.set_status_msg(status_str.into());
    }
}
