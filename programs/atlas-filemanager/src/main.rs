slint::include_modules!();

pub mod app_state;
pub mod fs;
pub mod utils;
pub mod callbacks;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::app_state::AppState;
use crate::fs::dir_reader::refresh_dir;

fn main() -> Result<(), slint::PlatformError> {
    let mut initial_path = PathBuf::from("/home/debian/Desktop/Sumer-OS-Portable");
    if !initial_path.exists() {
        initial_path = PathBuf::from("/home/debian");
        if !initial_path.exists() {
            initial_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        }
    }

    let state = Arc::new(Mutex::new(AppState {
        current_path: initial_path,
        history: Vec::new(),
        clipboard: None,
    }));

    let app = AppWindow::new()?;

    // Register callbacks using our modular design
    callbacks::navigation::register(&app, Arc::clone(&state));
    callbacks::selection::register(&app, Arc::clone(&state));
    callbacks::crud::register(&app, Arc::clone(&state));
    callbacks::preview::register(&app, Arc::clone(&state));
    callbacks::clipboard::register(&app, Arc::clone(&state));

    // Initial directory read
    refresh_dir(&app, &state);

    app.run()
}
