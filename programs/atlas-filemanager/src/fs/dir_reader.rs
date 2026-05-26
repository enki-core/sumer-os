use std::fs;
use std::sync::Mutex;
use slint::{SharedString, VecModel, ModelRc};

use crate::{AppWindow, FileItem};
use crate::app_state::AppState;
use crate::utils::time::format_system_time;
use crate::utils::format::format_size;
use crate::fs::path_utils::generate_path_segments;
use crate::fs::sorter::sort_file_items;

pub fn refresh_dir(app: &AppWindow, state: &Mutex<AppState>) {
    let (current_path, search_query) = {
        let s = state.lock().unwrap();
        (s.current_path.clone(), app.get_search_query().to_string())
    };

    let sort_mode = app.get_sort_mode().to_string();
    let show_hidden = app.get_show_hidden();

    app.set_is_loading(true);
    app.set_current_path(SharedString::from(current_path.to_string_lossy().to_string()));

    // Generate and set path segments (Breadcrumbs)
    let segments = generate_path_segments(&current_path);
    let segments_model = std::rc::Rc::new(VecModel::from(segments));
    app.set_path_segments(ModelRc::from(segments_model));

    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(&current_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Hidden files filter
            if !show_hidden && name.starts_with('.') {
                continue;
            }

            // Search filter (case-insensitive)
            if !search_query.is_empty() && !name.to_lowercase().contains(&search_query.to_lowercase()) {
                continue;
            }

            let metadata = fs::metadata(&path);
            let is_dir = path.is_dir();
            
            // Determine icon and file_type
            let (icon, file_type) = if is_dir {
                ("📁".to_string(), "folder".to_string())
            } else {
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                match ext.as_str() {
                    "txt" | "md" | "slint" | "rs" | "toml" | "json" | "css" | "sh" | "xml" | "html" | "conf" | "cfg" => {
                        ("📄".to_string(), "text".to_string())
                    }
                    "mp3" | "wav" | "ogg" | "flac" | "m4a" => {
                        ("🎵".to_string(), "audio".to_string())
                    }
                    "mp4" | "mkv" | "avi" | "mov" | "webm" => {
                        ("🎥".to_string(), "video".to_string())
                    }
                    "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" => {
                        ("🖼️".to_string(), "image".to_string())
                    }
                    "zip" | "tar" | "gz" | "xz" | "rar" | "7z" | "bz2" => {
                        ("📦".to_string(), "zip".to_string())
                    }
                    "run" | "bin" | "desktop" => {
                        ("⚙️".to_string(), "exe".to_string())
                    }
                    _ => ("❓".to_string(), "unknown".to_string()),
                }
            };

            // Format size
            let size_formatted = if is_dir {
                match fs::read_dir(&path) {
                    Ok(sub_entries) => {
                        let count = sub_entries.count();
                        format!("{} عناصر", count)
                    }
                    Err(_) => "0 عناصر".to_string(),
                }
            } else if let Ok(ref meta) = metadata {
                format_size(meta.len())
            } else {
                "-".to_string()
            };

            // Format modification time
            let modified_formatted = if let Ok(ref meta) = metadata {
                if let Ok(time) = meta.modified() {
                    format_system_time(time)
                } else {
                    "غير معروف".to_string()
                }
            } else {
                "غير معروف".to_string()
            };

            items.push(FileItem {
                name: SharedString::from(name),
                path: SharedString::from(path.to_string_lossy().to_string()),
                is_dir,
                icon: SharedString::from(icon),
                size_formatted: SharedString::from(size_formatted),
                modified_formatted: SharedString::from(modified_formatted),
                file_type: SharedString::from(file_type),
            });
        }
    }

    // Sort items
    sort_file_items(&mut items, &sort_mode);

    let items_model = std::rc::Rc::new(VecModel::from(items));
    app.set_files(ModelRc::from(items_model));

    // Update history button state
    let can_back = {
        let s = state.lock().unwrap();
        !s.history.is_empty()
    };
    app.set_can_go_back(can_back);

    app.set_is_loading(false);
}

pub fn load_text_preview(app: &AppWindow, file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let preview: String = content.chars().take(2000).collect();
            app.set_text_preview(SharedString::from(preview));
            app.set_show_text_preview(true);
        }
        Err(e) => {
            app.set_status_msg(SharedString::from(format!("فشل قراءة الملف: {}", e)));
            app.set_status_color(slint::Color::from_rgb_u8(239, 68, 68));
        }
    }
}
