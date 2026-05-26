use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use slint::{ComponentHandle, SharedString};

use crate::AppWindow;
use crate::app_state::AppState;
use crate::fs::dir_reader::refresh_dir;
use crate::fs::launcher::launch_file;

pub fn register(app: &AppWindow, state: Arc<Mutex<AppState>>) {
    // 5. Select Item Callback
    let app_weak = app.as_weak();
    app.on_select_item(move |item| {
        let app = app_weak.unwrap();
        app.set_selected_item(item.clone());
        app.set_has_selected(true);
    });

    // 6. Open Item Callback
    let app_weak = app.as_weak();
    let state_clone = Arc::clone(&state);
    app.on_open_item(move |item| {
        let app = app_weak.unwrap();
        if item.is_dir {
            let mut s = state_clone.lock().unwrap();
            let old_path = s.current_path.clone();
            s.history.push(old_path);
            s.current_path = PathBuf::from(item.path.as_str());
            drop(s);
            app.set_search_query(SharedString::from(""));
            app.set_has_selected(false);
            refresh_dir(&app, &state_clone);
        } else {
            launch_file(item.path.as_str());
        }
    });
}
