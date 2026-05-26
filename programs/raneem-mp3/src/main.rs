slint::include_modules!();

mod audio;
mod app;

fn main() -> Result<(), slint::PlatformError> {
    let main_app = AppWindow::new()?;
    
    // Setup all event handlers and keep the returned timer alive
    let _timer = app::setup(&main_app);
    
    // Start main Slint event loop
    main_app.run()
}
