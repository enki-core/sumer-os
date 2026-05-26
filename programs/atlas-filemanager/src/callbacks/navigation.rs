use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use slint::{ComponentHandle, SharedString};

use crate::AppWindow;
use crate::app_state::AppState;
use crate::fs::dir_reader::refresh_dir;

pub fn register(app: &AppWindow, state: Arc<Mutex<AppState>>) {
    // 1. Go Back Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_go_back(move || {
        let app = app_weak.unwrap();
        let mut s = state_clone.lock().unwrap();
        if let Some(prev) = s.history.pop() {
            s.current_path = prev;
            drop(s);
            app.set_has_selected(false);
            refresh_dir(&app, &state_clone);
        }
    });

    // 2. Go Up Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_go_up(move || {
        let app = app_weak.unwrap();
        let mut s = state_clone.lock().unwrap();
        if let Some(parent) = s.current_path.parent() {
            let parent_buf = parent.to_path_buf();
            let old_path = s.current_path.clone();
            s.history.push(old_path);
            s.current_path = parent_buf;
            drop(s);
            app.set_has_selected(false);
            refresh_dir(&app, &state_clone);
        }
    });

    // 3. Refresh Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_refresh(move || {
        let app = app_weak.unwrap();
        refresh_dir(&app, &state_clone);
    });

    // 4. Change Directory Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_change_directory(move |val| {
        let app = app_weak.unwrap();
        let target = PathBuf::from(val.as_str());
        if target.exists() && target.is_dir() {
            let mut s = state_clone.lock().unwrap();
            let old_path = s.current_path.clone();
            s.history.push(old_path);
            s.current_path = target;
            drop(s);
            app.set_has_selected(false);
            refresh_dir(&app, &state_clone);
        } else {
            app.set_status_msg(SharedString::from("المسار غير موجود أو ليس مجلداً"));
            app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
            let current = {
                let s = state_clone.lock().unwrap();
                s.current_path.to_string_lossy().to_string()
            };
            app.set_current_path(SharedString::from(current));
        }
    });
}
