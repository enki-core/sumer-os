use std::sync::{Arc, Mutex};
use slint::ComponentHandle;

use crate::AppWindow;
use crate::app_state::AppState;
use crate::fs::dir_reader::load_text_preview;

pub fn register(app: &AppWindow, _state: Arc<Mutex<AppState>>) {
    // 10. Load Text Preview Callback
    let app_weak = app.as_weak();
    app.on_load_text_preview(move |item| {
        let app = app_weak.unwrap();
        load_text_preview(&app, item.path.as_str());
    });
}
