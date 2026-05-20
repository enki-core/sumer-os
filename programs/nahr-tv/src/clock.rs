use crate::models::MainWindow;
use std::process::Command;

fn get_formatted_time() -> String {
    Command::new("date")
        .arg("+%I:%M %p")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string().to_uppercase())
        .unwrap_or_else(|_| "12:00 PM".to_string())
}

pub fn start_clock_sync(weak: slint::Weak<MainWindow>) -> slint::Timer {
    if let Some(window) = weak.upgrade() {
        window.set_current_time(get_formatted_time().into());
    }

    let timer = slint::Timer::default();
    let weak_clone = weak.clone();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(15), move || {
        if let Some(window) = weak_clone.upgrade() {
            window.set_current_time(get_formatted_time().into());
        }
    });
    timer
}
