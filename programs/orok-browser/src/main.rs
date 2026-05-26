slint::include_modules!();

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};

enum BrowserCommand {
    Navigate(String),
    Click(f32, f32),
    Reload,
    GoBack,
    GoForward,
}

struct Tab {
    title: String,
    url: String,
    history: Vec<String>,
    history_idx: usize,
    page_title: String,
    page_content: String,
}

struct BrowserState {
    tabs: Vec<Tab>,
    active_tab_idx: usize,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Initialize state with a single tab pointing to the Sumer OS Portal
    let initial_tab = Tab {
        title: "بوابة سومر".to_string(),
        url: "https://sumer.os".to_string(),
        history: vec!["https://sumer.os".to_string()],
        history_idx: 0,
        page_title: "بوابة سومر الوطنية".to_string(),
        page_content: "".to_string(),
    };

    let state = Arc::new(Mutex::new(BrowserState {
        tabs: vec![initial_tab],
        active_tab_idx: 0,
    }));

    // Initialize UI Tab Model
    update_tabs_model(&ui, &state.lock().unwrap());

    // Channels for communicating with Headless Chrome thread
    let (tx, rx) = channel::<BrowserCommand>();
    let ui_weak = ui.as_weak();

    // Spawn Headless Browser Thread (Chromium-based Opera)
    thread::spawn(move || {
        let options = LaunchOptions::default_builder()
            .path(Some(PathBuf::from("/usr/bin/opera")))
            .window_size(Some((1280, 800)))
            .headless(true)
            .sandbox(false)
            .build()
            .expect("Failed to build launch options");

        let browser = Browser::new(options).expect("Failed to launch Opera");
        let tab = browser.new_tab().expect("Failed to open tab");

        let _ = tab.navigate_to("https://sumer.os");

        loop {
            // 1. Process pending commands
            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    BrowserCommand::Navigate(url) => {
                        let _ = tab.navigate_to(&url);
                    }
                    BrowserCommand::Click(nx, ny) => {
                        let x = (nx * 1280.0) as f64;
                        let y = (ny * 800.0) as f64;
                        let _ = tab.click_point(headless_chrome::browser::tab::point::Point { x, y });
                    }
                    BrowserCommand::Reload => {
                        let _ = tab.reload(true, None);
                    }
                    BrowserCommand::GoBack => {
                        let _ = tab.evaluate("window.history.back()", false);
                    }
                    BrowserCommand::GoForward => {
                        let _ = tab.evaluate("window.history.forward()", false);
                    }
                }
            }

            // 2. Retrieve Page Title and URL dynamically
            let mut current_title = String::new();
            let mut current_url = String::new();

            if let Ok(title_val) = tab.evaluate("document.title", false) {
                if let Some(title_str) = title_val.value.and_then(|v| v.as_str().map(|s| s.to_string())) {
                    current_title = title_str;
                }
            }

            if let Ok(url_val) = tab.evaluate("window.location.href", false) {
                if let Some(url_str) = url_val.value.and_then(|v| v.as_str().map(|s| s.to_string())) {
                    current_url = url_str;
                }
            }

            if !current_title.is_empty() || !current_url.is_empty() {
                let ui_inner = ui_weak.clone();
                let u_title = current_title.clone();
                let u_url = current_url.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_inner.upgrade() {
                        if !u_title.is_empty() {
                            ui.set_page_title(u_title.into());
                        }
                        if !u_url.is_empty() {
                            ui.set_current_url(u_url.into());
                        }
                    }
                });
            }

            if let Ok(jpeg_data) = tab.capture_screenshot(
                CaptureScreenshotFormatOption::Jpeg,
                Some(75),
                None,
                true,
            ) {
                if let Ok(dynamic_image) = image::load_from_memory_with_format(&jpeg_data, image::ImageFormat::Jpeg) {
                    let rgba_image = dynamic_image.to_rgba8();
                    let width = rgba_image.width();
                    let height = rgba_image.height();
                    let raw_pixels = rgba_image.into_raw();

                    let ui_inner = ui_weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_inner.upgrade() {
                            let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                                &raw_pixels,
                                width,
                                height,
                            );
                            ui.set_web_view_image(slint::Image::from_rgba8(buffer));
                            ui.set_is_loading(false);
                        }
                    });
                }
            }

            // Throttle loop for smooth rendering without high CPU usage
            thread::sleep(Duration::from_millis(80));
        }
    });

    // 1. Navigation Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_navigate(move |url| {
        let url = url.to_string();
        if let Some(_) = ui_weak.upgrade() {
            let s = state_clone.lock().unwrap();
            let active_idx = s.active_tab_idx;
            drop(s);
            navigate_tab(ui_weak.clone(), state_clone.clone(), active_idx, url.clone(), tx_clone.clone());
        }
    });

    // 2. Go Back Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_back(move || {
        if let Some(_) = ui_weak.upgrade() {
            let s = state_clone.lock().unwrap();
            let active_idx = s.active_tab_idx;
            let has_back = s.tabs[active_idx].history_idx > 0;
            drop(s);
            
            if has_back {
                let mut s = state_clone.lock().unwrap();
                s.tabs[active_idx].history_idx -= 1;
                let prev_url = s.tabs[active_idx].history[s.tabs[active_idx].history_idx].clone();
                drop(s);
                
                navigate_tab(ui_weak.clone(), state_clone.clone(), active_idx, prev_url, tx_clone.clone());
                let _ = tx_clone.send(BrowserCommand::GoBack);
            }
        }
    });

    // 3. Go Forward Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_forward(move || {
        if let Some(_) = ui_weak.upgrade() {
            let s = state_clone.lock().unwrap();
            let active_idx = s.active_tab_idx;
            let has_fwd = s.tabs[active_idx].history_idx + 1 < s.tabs[active_idx].history.len();
            drop(s);
            
            if has_fwd {
                let mut s = state_clone.lock().unwrap();
                s.tabs[active_idx].history_idx += 1;
                let next_url = s.tabs[active_idx].history[s.tabs[active_idx].history_idx].clone();
                drop(s);
                
                navigate_tab(ui_weak.clone(), state_clone.clone(), active_idx, next_url, tx_clone.clone());
                let _ = tx_clone.send(BrowserCommand::GoForward);
            }
        }
    });

    // 4. Reload Event
    let tx_clone = tx.clone();
    ui.on_reload(move || {
        let _ = tx_clone.send(BrowserCommand::Reload);
    });

    // 5. Add Tab Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_add_tab(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut s = state_clone.lock().unwrap();
            
            let new_tab = Tab {
                title: "بوابة سومر".to_string(),
                url: "https://sumer.os".to_string(),
                history: vec!["https://sumer.os".to_string()],
                history_idx: 0,
                page_title: "بوابة سومر الوطنية".to_string(),
                page_content: "".to_string(),
            };
            
            s.tabs.push(new_tab);
            s.active_tab_idx = s.tabs.len() - 1;
            
            ui.set_current_url("https://sumer.os".into());
            ui.set_page_title("بوابة سومر الوطنية".into());
            ui.set_page_content("".into());
            ui.set_is_loading(false);
            
            update_tabs_model(&ui, &s);
            let _ = tx_clone.send(BrowserCommand::Navigate("https://sumer.os".to_string()));
        }
    });

    // 6. Close Tab Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_close_tab(move |idx| {
        let idx = idx as usize;
        if let Some(ui) = ui_weak.upgrade() {
            let mut s = state_clone.lock().unwrap();
            if s.tabs.len() > 1 && idx < s.tabs.len() {
                s.tabs.remove(idx);
                
                if s.active_tab_idx >= s.tabs.len() {
                    s.active_tab_idx = s.tabs.len() - 1;
                } else if s.active_tab_idx == idx && idx > 0 {
                    s.active_tab_idx = idx - 1;
                }
                
                let active_idx = s.active_tab_idx;
                let active_url = s.tabs[active_idx].url.clone();
                let active_title = s.tabs[active_idx].page_title.clone();
                let active_content = s.tabs[active_idx].page_content.clone();
                
                ui.set_current_url(active_url.clone().into());
                ui.set_page_title(active_title.into());
                ui.set_page_content(active_content.into());
                
                update_tabs_model(&ui, &s);
                let _ = tx_clone.send(BrowserCommand::Navigate(active_url));
            }
        }
    });

    // 7. Select Tab Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_select_tab(move |idx| {
        let idx = idx as usize;
        if let Some(ui) = ui_weak.upgrade() {
            let mut s = state_clone.lock().unwrap();
            if idx < s.tabs.len() {
                s.active_tab_idx = idx;
                
                let active_url = s.tabs[idx].url.clone();
                let active_title = s.tabs[idx].page_title.clone();
                let active_content = s.tabs[idx].page_content.clone();
                
                ui.set_current_url(active_url.clone().into());
                ui.set_page_title(active_title.into());
                ui.set_page_content(active_content.into());
                
                update_tabs_model(&ui, &s);
                let _ = tx_clone.send(BrowserCommand::Navigate(active_url));
            }
        }
    });

    // 8. Quick Link Clicked Event
    let ui_weak = ui.as_weak();
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    ui.on_quick_link_clicked(move |url| {
        let url = url.to_string();
        if let Some(_) = ui_weak.upgrade() {
            let s = state_clone.lock().unwrap();
            let active_idx = s.active_tab_idx;
            drop(s);
            
            navigate_tab(ui_weak.clone(), state_clone.clone(), active_idx, url, tx_clone.clone());
        }
    });

    // 9. Open External Browser Event
    ui.on_open_external(move |url| {
        let url_str = url.to_string();
        println!("[Orok] Opening externally: {}", url_str);
        let mut target_url = url_str.trim().to_string();
        if !target_url.starts_with("http://") && !target_url.starts_with("https://") {
            if target_url.contains('.') {
                target_url = format!("https://{}", target_url);
            } else {
                target_url = format!("https://www.google.com/search?q={}", percent_encode(&target_url));
            }
        }
        let _ = open::that(target_url);
    });

    // 10. Handle Mouse Click inside rendered Web View
    let tx_clone = tx.clone();
    ui.on_handle_mouse_click(move |nx, ny| {
        let _ = tx_clone.send(BrowserCommand::Click(nx, ny));
    });

    println!("==========================================");
    println!("   ⛵ Orok Browser (v0.9.8) is running   ");
    println!("==========================================");

    ui.run()
}

fn update_tabs_model(ui: &AppWindow, state: &BrowserState) {
    let slint_tabs: Vec<TabInfo> = state.tabs.iter().enumerate().map(|(idx, tab)| {
        TabInfo {
            title: tab.title.clone().into(),
            url: tab.url.clone().into(),
            active: idx == state.active_tab_idx,
        }
    }).collect();
    
    let model = rc_model_from_vec(slint_tabs);
    ui.set_tabs(model.into());
    ui.set_active_tab_index(state.active_tab_idx as i32);
}

fn rc_model_from_vec<T: 'static + Clone>(v: Vec<T>) -> slint::ModelRc<T> {
    let vec_model = slint::VecModel::from(v);
    slint::ModelRc::from(std::rc::Rc::new(vec_model))
}

fn percent_encode(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.as_bytes() {
        match *b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

fn encode_url_non_ascii(url: &str) -> String {
    let mut output = String::new();
    for c in url.chars() {
        if c.is_ascii() {
            output.push(c);
        } else {
            let mut buf = [0; 4];
            let bytes = c.encode_utf8(&mut buf).as_bytes();
            for b in bytes {
                output.push_str(&format!("%{:02X}", b));
            }
        }
    }
    output
}

fn navigate_tab(
    ui_weak: slint::Weak<AppWindow>,
    state: Arc<Mutex<BrowserState>>,
    tab_idx: usize,
    url: String,
    tx: Sender<BrowserCommand>,
) {
    let mut cleaned_url = url.trim().to_string();
    if cleaned_url.is_empty() {
        cleaned_url = "https://sumer.os".to_string();
    }
    
    if !cleaned_url.starts_with("http://") && !cleaned_url.starts_with("https://") {
        if cleaned_url.contains('.') {
            cleaned_url = format!("https://{}", cleaned_url);
        } else {
            let encoded_query = percent_encode(&cleaned_url);
            cleaned_url = format!("https://www.google.com/search?q={}", encoded_query);
        }
    }
    
    let encoded_url = encode_url_non_ascii(&cleaned_url);
    println!("[Orok] Tab {} Navigating to: {}", tab_idx, encoded_url);
    
    if let Some(ui) = ui_weak.upgrade() {
        let mut s = state.lock().unwrap();
        if tab_idx < s.tabs.len() {
            let tab = &mut s.tabs[tab_idx];
            
            if tab.history_idx < tab.history.len() {
                tab.history.truncate(tab.history_idx + 1);
            }
            tab.history.push(encoded_url.clone());
            tab.history_idx = tab.history.len() - 1;
            
            tab.url = encoded_url.clone();
            
            if s.active_tab_idx == tab_idx {
                ui.set_current_url(cleaned_url.clone().into());
                ui.set_is_loading(true);
            }
            
            let _ = tx.send(BrowserCommand::Navigate(encoded_url));
        }
    }
}