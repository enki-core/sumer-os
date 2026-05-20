use crate::models::MainWindow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_hide_timer(
    weak: slint::Weak<MainWindow>,
    session_counter: Arc<Mutex<u32>>,
    setter: fn(&MainWindow, bool),
) {
    let mut session = session_counter.lock().unwrap();
    *session += 1;
    let current_session = *session;
    drop(session);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let session = session_counter.lock().unwrap();
        if *session == current_session {
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = weak.upgrade() {
                    setter(&window, false);
                }
            });
        }
    });
}
