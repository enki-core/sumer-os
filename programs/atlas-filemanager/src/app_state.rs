use std::path::PathBuf;

pub struct AppState {
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
    pub clipboard: Option<(PathBuf, bool)>, // (source_path, is_cut)
}
