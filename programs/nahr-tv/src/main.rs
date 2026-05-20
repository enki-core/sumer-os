mod app;
mod launcher;
mod bluetooth;
mod audio;
mod timers;
mod clock;
mod models;

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}
