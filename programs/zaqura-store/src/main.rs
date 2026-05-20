mod models;
mod launcher;
mod store;
mod handlers;
mod app;

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}
