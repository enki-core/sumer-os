use std::sync::{Arc, Mutex};
use slint::{ComponentHandle, SharedString};

use crate::AppWindow;
use crate::app_state::AppState;
use crate::fs::dir_reader::refresh_dir;
use crate::fs::writer::{create_item, rename_item, delete_item};

pub fn register(app: &AppWindow, state: Arc<Mutex<AppState>>) {
    // 7. Delete Item Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_delete_item(move |item| {
        let app = app_weak.unwrap();
        match delete_item(item.path.as_str()) {
            Ok(_) => {
                app.set_status_msg(SharedString::from("تم حذف العنصر بنجاح"));
                app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
                app.set_has_selected(false);
                refresh_dir(&app, &state_clone);
            }
            Err(e) => {
                app.set_status_msg(SharedString::from(format!("فشل حذف العنصر: {}", e)));
                app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
            }
        }
    });

    // 8. Rename Item Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_rename_item(move |item, new_name| {
        let app = app_weak.unwrap();
        match rename_item(item.path.as_str(), new_name.as_str()) {
            Ok(_) => {
                app.set_status_msg(SharedString::from("تم تعديل الاسم بنجاح"));
                app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
                app.set_has_selected(false);
                refresh_dir(&app, &state_clone);
            }
            Err(e) => {
                app.set_status_msg(SharedString::from(format!("فشل تعديل الاسم: {}", e)));
                app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
            }
        }
    });

    // 9. Create Item Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_create_item(move |name, is_folder| {
        let app = app_weak.unwrap();
        let current_path = {
            let s = state_clone.lock().unwrap();
            s.current_path.clone()
        };
        match create_item(&current_path, name.as_str(), is_folder) {
            Ok(_) => {
                let msg = if is_folder { "تم إنشاء المجلد بنجاح" } else { "تم إنشاء الملف بنجاح" };
                app.set_status_msg(SharedString::from(msg));
                app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
                refresh_dir(&app, &state_clone);
            }
            Err(e) => {
                let msg = if is_folder { format!("فشل إنشاء المجلد: {}", e) } else { format!("فشل إنشاء الملف: {}", e) };
                app.set_status_msg(SharedString::from(msg));
                app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
            }
        }
    });
}
