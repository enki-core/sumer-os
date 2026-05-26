slint::include_modules!();

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use std::thread;

// Helper to find the generated page file in the temp directory (supporting different zero-paddings)
fn find_generated_page(temp_dir: &str, page_num: i32) -> Option<PathBuf> {
    let dir = fs::read_dir(temp_dir).ok()?;
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "png" {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            // Check if it ends with the page pattern (e.g. "-1.png", "-01.png", "-001.png")
                            let suffix = format!("-{}.png", page_num);
                            let suffix_z = format!("-0{}.png", page_num);
                            let suffix_zz = format!("-00{}.png", page_num);
                            let suffix_zzz = format!("-000{}.png", page_num);
                            
                            // Check if name ends with any of these and does not contain "dark"
                            if !name.contains("dark") && (
                                name.ends_with(&suffix) 
                                || name.ends_with(&suffix_z) 
                                || name.ends_with(&suffix_zz)
                                || name.ends_with(&suffix_zzz)
                            ) {
                                return Some(path);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// Render page image using pdftoppm (and mogrify/convert for dark mode)
fn render_page_image(app: &AppWindow, file_path: &str, page_num: i32, _total_pages: i32, is_dark: bool) {
    app.set_is_loading_page(true);
    
    let safe_name: String = file_path.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let temp_dir = format!("/tmp/mesh-pdf/{}", safe_name);
    fs::create_dir_all(&temp_dir).ok();

    let app_weak = app.as_weak();
    let file_path_clone = file_path.to_string();
    
    thread::spawn(move || {
        // Check if page already rendered
        let original_path = find_generated_page(&temp_dir, page_num);
        let final_image_path = if let Some(path) = original_path {
            path
        } else {
            // Render the single page
            let prefix = format!("{}/page", temp_dir);
            let _status = Command::new("pdftoppm")
                .arg("-png")
                .arg("-r")
                .arg("150")
                .arg("-f")
                .arg(page_num.to_string())
                .arg("-l")
                .arg(page_num.to_string())
                .arg(&file_path_clone)
                .arg(&prefix)
                .status();

            find_generated_page(&temp_dir, page_num).unwrap_or_else(|| PathBuf::from(""))
        };

        if final_image_path.exists() && final_image_path.is_file() {
            let display_path = if is_dark {
                // Negate colors for dark reading mode
                let dark_path = final_image_path.with_file_name(format!("page-dark-{}.png", page_num));
                if !dark_path.exists() {
                    let _status = Command::new("convert")
                        .arg(&final_image_path)
                        .arg("-negate")
                        .arg(&dark_path)
                        .status();
                }
                if dark_path.exists() {
                    dark_path
                } else {
                    final_image_path
                }
            } else {
                final_image_path
            };

            if display_path.exists() {
                let display_path_str = display_path.to_string_lossy().to_string();
                app_weak.upgrade_in_event_loop(move |app| {
                    if let Ok(slint_img) = slint::Image::load_from_path(Path::new(&display_path_str)) {
                        app.set_current_page_image(slint_img);
                    }
                    app.set_is_loading_page(false);
                }).unwrap();
                return;
            }
        }

        // Fallback
        app_weak.upgrade_in_event_loop(move |app| {
            app.set_is_loading_page(false);
        }).unwrap();
    });
}

// Open PDF File logic: parses metadata and loads it
fn open_pdf_file(app: &AppWindow, file_path: &str) {
    if !Path::new(file_path).exists() {
        return;
    }

    let output = Command::new("pdfinfo")
        .arg(file_path)
        .output();

    let mut title = String::new();
    let mut author = String::new();
    let mut pages = 0;
    let mut creation_date = String::new();
    let mut file_size = String::new();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim().to_string();
                match key {
                    "Title" => title = val,
                    "Author" => author = val,
                    "Pages" => pages = val.parse::<i32>().unwrap_or(0),
                    "CreationDate" => creation_date = val,
                    "File size" => file_size = val,
                    _ => {}
                }
            }
        }
    }

    if pages <= 0 {
        return;
    }

    if title.is_empty() {
        if let Some(filename) = Path::new(file_path).file_name() {
            title = filename.to_string_lossy().into_owned();
        } else {
            title = "مستند بي دي إف".to_string();
        }
    }
    if author.is_empty() {
        author = "غير معروف".to_string();
    }

    // Set state
    app.set_current_file_path(file_path.into());
    app.set_current_page(1);
    app.set_total_pages(pages);
    app.set_has_file_loaded(true);
    app.set_is_dark_mode(false);
    app.set_zoom_level(1.0);
    app.set_search_results(slint::ModelRc::new(slint::VecModel::default()));
    app.set_search_query("".into());

    let metadata = PdfMetadata {
        title: title.into(),
        author: author.into(),
        pages,
        file_size: file_size.into(),
        creation_date: creation_date.into(),
        file_path: file_path.into(),
    };
    app.set_current_metadata(metadata);

    // Save to history file
    add_to_history(file_path);
    load_history_into_app(app);

    // Load first page
    render_page_image(app, file_path, 1, pages, false);
}

// History handlers
fn get_history_file_path() -> PathBuf {
    Path::new("/home/debian/Desktop/Sumer-OS-Portable/programs/mesh-pdf").join("recent.txt")
}

fn add_to_history(file_path: &str) {
    let history_file = get_history_file_path();
    let mut history = Vec::new();
    
    if let Ok(content) = fs::read_to_string(&history_file) {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                history.push(trimmed.to_string());
            }
        }
    }

    let path = Path::new(file_path);
    let name = path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "مستند بي دي إف".to_string());

    let date_str = if let Ok(output) = Command::new("date").arg("+%Y-%m-%d %H:%M").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "2026-05-26 17:00".to_string()
    };

    let entry = format!("{}|{}|{}", file_path, name, date_str);

    history.retain(|line| {
        let parts: Vec<&str> = line.split('|').collect();
        parts.is_empty() || parts[0] != file_path
    });

    history.insert(0, entry);
    history.truncate(10);

    let new_content = history.join("\n");
    fs::write(&history_file, new_content).ok();
}

fn load_history_into_app(app: &AppWindow) {
    let history_file = get_history_file_path();
    let mut recent_files = Vec::new();

    if let Ok(content) = fs::read_to_string(&history_file) {
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                recent_files.push(RecentFile {
                    path: parts[0].into(),
                    name: parts[1].into(),
                    date: parts[2].into(),
                });
            }
        }
    }

    let model = slint::ModelRc::new(slint::VecModel::from(recent_files));
    app.set_recent_files(model);
}

fn clear_history_in_file() {
    let history_file = get_history_file_path();
    fs::write(&history_file, "").ok();
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    load_history_into_app(&app);

    let app_weak = app.as_weak();
    
    // Callback: Open Native Zenity File Dialog in background thread
    app.on_open_file_dialog(move || {
        let app_weak_clone = app_weak.clone();
        thread::spawn(move || {
            let output = Command::new("zenity")
                .arg("--file-selection")
                .arg("--file-filter=PDF Files | *.pdf *.PDF")
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        app_weak_clone.upgrade_in_event_loop(move |app| {
                            open_pdf_file(&app, &path_str);
                        }).unwrap();
                    }
                }
            }
        });
    });

    let app_weak = app.as_weak();
    app.on_open_file(move |path| {
        if let Some(app) = app_weak.upgrade() {
            open_pdf_file(&app, path.as_str());
        }
    });

    let app_weak = app.as_weak();
    app.on_change_page(move |page_num| {
        if let Some(app) = app_weak.upgrade() {
            let total = app.get_total_pages();
            if page_num >= 1 && page_num <= total {
                app.set_current_page(page_num);
                let path = app.get_current_file_path();
                let is_dark = app.get_is_dark_mode();
                render_page_image(&app, path.as_str(), page_num, total, is_dark);
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_jump_to_page(move |val| {
        if let Some(app) = app_weak.upgrade() {
            if let Ok(page_num) = val.trim().parse::<i32>() {
                let total = app.get_total_pages();
                if page_num >= 1 && page_num <= total {
                    app.set_current_page(page_num);
                    let path = app.get_current_file_path();
                    let is_dark = app.get_is_dark_mode();
                    render_page_image(&app, path.as_str(), page_num, total, is_dark);
                }
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_toggle_dark_mode(move || {
        if let Some(app) = app_weak.upgrade() {
            let is_dark = !app.get_is_dark_mode();
            app.set_is_dark_mode(is_dark);
            
            let path = app.get_current_file_path();
            let page_num = app.get_current_page();
            let total = app.get_total_pages();
            render_page_image(&app, path.as_str(), page_num, total, is_dark);
        }
    });

    let app_weak = app.as_weak();
    app.on_clear_history(move || {
        if let Some(app) = app_weak.upgrade() {
            clear_history_in_file();
            load_history_into_app(&app);
        }
    });

    // Callback: Text Search
    let app_weak = app.as_weak();
    app.on_search_text(move |query| {
        let app_weak_clone = app_weak.clone();
        let query_str = query.as_str().to_string();
        
        let file_path = if let Some(app) = app_weak.upgrade() {
            app.get_current_file_path().as_str().to_string()
        } else {
            String::new()
        };
        
        thread::spawn(move || {
            let mut results = Vec::new();
            
            if !file_path.is_empty() {
                    // Extract text with pdftotext
                    let output = Command::new("pdftotext")
                        .arg(&file_path)
                        .arg("-")
                        .output();

                    if let Ok(output) = output {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        // pdftotext splits pages by form-feed character (\x0c)
                        let pages: Vec<&str> = stdout.split('\x0c').collect();
                        
                        let query_lower = query_str.to_lowercase();
                        
                        for (i, page_text) in pages.iter().enumerate() {
                            let page_num = (i + 1) as i32;
                            let text_lower = page_text.to_lowercase();
                            
                            if let Some(index) = text_lower.find(&query_lower) {
                                // Extract short snippet
                                let start = if index > 35 { index - 35 } else { 0 };
                                let end = std::cmp::min(index + query_lower.len() + 35, page_text.len());
                                
                                let snippet_slice = &page_text[start..end];
                                let clean_snippet = snippet_slice.replace('\n', " ").trim().to_string();
                                let formatted_snippet = format!("... {} ...", clean_snippet);
                                
                                results.push(SearchResult {
                                    page: page_num,
                                    snippet: formatted_snippet.into(),
                                });
                                
                                // Limit search results to 60 matches to avoid UI clutter
                                if results.len() >= 60 {
                                    break;
                                }
                            }
                    }
                }
            }
            
            app_weak_clone.upgrade_in_event_loop(move |app| {
                let model = slint::ModelRc::new(slint::VecModel::from(results));
                app.set_search_results(model);
                app.set_is_searching(false);
            }).unwrap();
        });
    });

    app.run()
}
