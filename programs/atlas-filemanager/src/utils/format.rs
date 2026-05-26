pub fn format_size(len: u64) -> String {
    if len < 1024 {
        format!("{} B", len)
    } else if len < 1024 * 1024 {
        format!("{:.1} KB", len as f64 / 1024.0)
    } else if len < 1024 * 1024 * 1024 {
        format!("{:.1} MB", len as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", len as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
