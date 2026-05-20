fn main() {
    // Compile app.slint -> generates app.rs in OUT_DIR, which resolves all imports inside ui/
    slint_build::compile("ui/app.slint").unwrap();
}
