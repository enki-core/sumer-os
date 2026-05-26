// Linka Link2phone - Rust Main Entry Point
// Handles Slint UI initialization and background threads for call simulation timers.

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // Handle backspace on dialed number
    let app_weak = app.as_weak();
    app.on_backspace_pressed(move || {
        if let Some(app) = app_weak.upgrade() {
            let mut num = app.get_dialed_number().to_string();
            if !num.is_empty() {
                num.pop();
                app.set_dialed_number(num.into());
            }
        }
    });

    // Background thread to increment the call duration timer when calling
    let app_weak = app.as_weak();
    std::thread::spawn(move || {
        let mut seconds = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if let Some(app) = app_weak.upgrade() {
                if app.get_is_calling() {
                    seconds += 1;
                    let mins = seconds / 60;
                    let secs = seconds % 60;
                    let time_str = format!("{:02}:{:02}", mins, secs);
                    app.set_call_timer(time_str.into());
                } else {
                    seconds = 0;
                    app.set_call_timer("00:00".into());
                }
            }
        }
    });

    app.run()
}
