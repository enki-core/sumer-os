use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use slint::{ComponentHandle, SharedString};

use crate::AppWindow;
use crate::app_state::AppState;
use crate::fs::dir_reader::refresh_dir;
use crate::fs::clipboard::paste_item;

pub fn register(app: &AppWindow, state: Arc<Mutex<AppState>>) {
    // 11. Copy Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_copy_item(move |item| {
        let app = app_weak.unwrap();
        let path = PathBuf::from(item.path.as_str());
        let mut s = state_clone.lock().unwrap();
        s.clipboard = Some((path.clone(), false));
        drop(s);
        app.set_clipboard_source_path(SharedString::from(path.to_string_lossy().to_string()));
        app.set_clipboard_is_cut(false);
        app.set_has_clipboard(true);
        app.set_status_msg(SharedString::from("تم نسخ الملف إلى الحافظة"));
        app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
    });

    // 12. Cut Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_cut_item(move |item| {
        let app = app_weak.unwrap();
        let path = PathBuf::from(item.path.as_str());
        let mut s = state_clone.lock().unwrap();
        s.clipboard = Some((path.clone(), true));
        drop(s);
        app.set_clipboard_source_path(SharedString::from(path.to_string_lossy().to_string()));
        app.set_clipboard_is_cut(true);
        app.set_has_clipboard(true);
        app.set_status_msg(SharedString::from("تم قص الملف إلى الحافظة"));
        app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
    });

    // 13. Paste Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_paste_item(move || {
        let app = app_weak.unwrap();
        let (clipboard, current_path) = {
            let s = state_clone.lock().unwrap();
            (s.clipboard.clone(), s.current_path.clone())
        };

        if let Some((src_path, is_cut)) = clipboard {
            match paste_item(&src_path.to_string_lossy(), &current_path.to_string_lossy(), is_cut) {
                Ok(_) => {
                    app.set_status_msg(SharedString::from("تمت العملية بنجاح"));
                    app.set_status_color(slint::Color::from_rgb_u8(16, 185, 129));
                    
                    if is_cut {
                        let mut s = state_clone.lock().unwrap();
                        s.clipboard = None;
                        drop(s);
                        app.set_has_clipboard(false);
                        app.set_clipboard_source_path(SharedString::default());
                    }
                    
                    refresh_dir(&app, &state_clone);
                }
                Err(e) => {
                    app.set_status_msg(SharedString::from(format!("فشل اللصق: {}", e)));
                    app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
                }
            }
        }
    });
}
