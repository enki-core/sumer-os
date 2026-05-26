slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // Weak reference to trigger startup fade-in animation
    let app_weak = app.as_weak();
    slint::Timer::single_shot(std::time::Duration::from_millis(150), move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_show_content(true);
            app.set_status_msg("بانتظار التحقق... أدخل PIN: 1234 أو كلمة المرور: admin".into());
        }
    });

    // Handle Authentication Callback
    let app_weak = app.as_weak();
    app.on_authenticate(move |mode, input| {
        let input_str = input.as_str().trim();
        println!("[Anu Lock] Authenticating mode: {}, input: {}", mode, input_str);

        match mode {
            0 => {
                // Fingerprint simulation always succeeds in this prototype
                true
            }
            1 => {
                // Face ID simulation always succeeds in this prototype
                true
            }
            2 => {
                // PIN authentication (Correct PIN is "1234")
                if input_str == "1234" {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_status_msg("تم التحقق من الرمز بنجاح!".into());
                    }
                    true
                } else {
                    false
                }
            }
            3 => {
                // Password authentication (Correct Password is "admin" or "sumer")
                if input_str == "admin" || input_str == "sumer" || input_str == "password" {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_status_msg("تم التحقق من كلمة المرور!".into());
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    });

    // String Helper Callbacks
    app.on_get_string_length(move |s| {
        s.as_str().chars().count() as i32
    });

    app.on_delete_last_char(move |s| {
        let mut chars = s.as_str().chars();
        chars.next_back();
        chars.as_str().into()
    });

    // Handle Closing Application
    let app_weak = app.as_weak();
    app.on_close_app(move || {
        println!("[Anu Lock] Closing lock screen...");
        if let Some(app) = app_weak.upgrade() {
            let _ = app.hide();
        }
    });

    app.run()
}
