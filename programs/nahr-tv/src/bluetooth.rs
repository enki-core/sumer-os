use crate::models::MainWindow;
use std::thread;
use std::time::Duration;

pub fn handle_bluetooth_click(weak: slint::Weak<MainWindow>, device_name: String) {
    if let Some(window) = weak.upgrade() {
        window.set_bluetooth_connecting(true);
        window.set_bluetooth_connection_status(format!("جاري الاقتران مع {}...", device_name).into());
    }

    let weak_clone = weak.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1500));
        let weak_s1 = weak_clone.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = weak_s1.upgrade() {
                window.set_bluetooth_connection_status("تم الاقتران بنجاح! 🔵".into());
            }
        });

        thread::sleep(Duration::from_millis(1000));
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = weak_clone.upgrade() {
                window.set_bluetooth_connecting(false);
                window.set_show_settings(false);
            }
        });
    });
}
