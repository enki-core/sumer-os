slint::include_modules!();
use slint::{ComponentHandle, VecModel, Model};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    
    // Initialize the dynamic Dock array in Rust
    let dock_icons = Rc::new(VecModel::from(vec![
        AppIcon { id: "files".into(), name: "Files".into(), icon: "📁".into(), bg_color: slint::Color::from_argb_encoded(0xff3b82f6), hover_bg_color: slint::Color::from_argb_encoded(0xff2563eb) },
        AppIcon { id: "console".into(), name: "Console".into(), icon: "💻".into(), bg_color: slint::Color::from_argb_encoded(0xff10b981), hover_bg_color: slint::Color::from_argb_encoded(0xff059669) },
        AppIcon { id: "tv".into(), name: "TV".into(), icon: "📺".into(), bg_color: slint::Color::from_argb_encoded(0xfff97316), hover_bg_color: slint::Color::from_argb_encoded(0xffea580c) },
    ]));
    main_window.set_dock_icons(dock_icons.clone().into());
    
    // Connect App Click Handler to Launch Standalone Binaries
    main_window.on_app_clicked(move |app_name| {
        let app_name = app_name.to_string();
        println!("App clicked in UI: {}", app_name);
        
        let binary_path = match app_name.as_str() {
            "TV" | "تلفاز نهر" | "nahr-tv" => "/usr/bin/nahr-tv",
            "Console" | "💻" | "شيل دجلة" => "/usr/bin/tigris-shell",
            "Store" | "zaqura-store" => "/usr/bin/zaqura-store",
            "Notepad" | "alwah-notepad" => "/usr/bin/alwah-notepad",
            "Calc" | "enki-calc" => "/usr/bin/enki-calc",
            "Browser" | "orok-browser" => "/programs/orok-browser/run",
            _ => return,
        };

        let mut actual_path = binary_path.to_string();
        if !std::path::Path::new(&actual_path).exists() {
            // Check in same parent directory (programs/)
            let relative_name = match app_name.as_str() {
                "TV" | "تلفاز نهر" | "nahr-tv" => "nahr-tv",
                "Console" | "💻" | "شيل دجلة" => "tigris-shell",
                "Store" | "zaqura-store" => "zaqura-store",
                "Notepad" | "alwah-notepad" => "alwah-notepad",
                "Calc" | "enki-calc" => "enki-calc",
                _ => "",
            };
            if !relative_name.is_empty() {
                let paths_to_check = vec![
                    format!("../programs/{}/target-alpine/release/{}", relative_name, relative_name),
                    format!("../{}/target-alpine/release/{}", relative_name, relative_name),
                    format!("../programs/{}/target/debug/{}", relative_name, relative_name),
                    format!("../{}/target/debug/{}", relative_name, relative_name),
                ];
                for p in paths_to_check {
                    if std::path::Path::new(&p).exists() {
                        actual_path = p;
                        break;
                    }
                }
            }
        }

        if std::path::Path::new(&actual_path).exists() {
            std::process::Command::new(&actual_path).spawn().ok();
        }
    });

    // Connect Drag & Drop Handlers
    let window_weak_drag = main_window.as_weak();
    let dock_icons_drag = dock_icons.clone();
    main_window.on_icon_drag_started(move |idx, _mx, _my| {
        if let Some(window) = window_weak_drag.upgrade() {
            if let Some(icon) = dock_icons_drag.row_data(idx as usize) {
                window.set_dragged_icon(icon);
                window.set_is_dragging_icon(true);
            }
        }
    });

    let window_weak_drag_moved = main_window.as_weak();
    let dock_icons_count = dock_icons.clone();
    main_window.on_icon_drag_moved(move |idx, delta_x, delta_y| {
        if let Some(window) = window_weak_drag_moved.upgrade() {
            // Predict the layout geometry!
            let total_icons = dock_icons_count.row_count();
            let size = window.window().size();
            let w_width = size.width as f32;
            let w_height = size.height as f32;
            
            // Start button (60px) + Separator (13px) + padding = 85px start.
            let dock_width = 85.0 + (total_icons as f32 * 60.0); 
            let dock_x = (w_width - dock_width) / 2.0;
            let dock_y = w_height - 72.0 - 24.0;
            
            let item_x = dock_x + 85.0 + (idx as f32 * 60.0);
            let item_y = dock_y + 12.0;
            
            window.set_drag_x(item_x + delta_x);
            window.set_drag_y(item_y + delta_y);
        }
    });

    let window_weak_drag_ended = main_window.as_weak();
    let dock_icons_drop = dock_icons.clone();
    main_window.on_icon_drag_ended(move |idx, _mx, _my| {
        if let Some(window) = window_weak_drag_ended.upgrade() {
            window.set_is_dragging_icon(false);
            
            let drop_x = window.get_drag_x();
            let drop_y = window.get_drag_y();
            let size = window.window().size();
            let w_height = size.height as f32;
            
            // If dropped significantly outside the Dock (e.g. above it)
            if drop_y < w_height - 150.0 {
                if let Some(icon) = dock_icons_drop.row_data(idx as usize) {
                    dock_icons_drop.remove(idx as usize);
                    if icon.id == "files" { 
                        window.set_desktop_icon0_x(drop_x);
                        window.set_desktop_icon0_y(drop_y);
                        window.set_show_desktop_files(true); 
                    }
                    else if icon.id == "console" { 
                        window.set_desktop_icon1_x(drop_x);
                        window.set_desktop_icon1_y(drop_y);
                        window.set_show_desktop_console(true); 
                    }
                    else if icon.id == "tv" { 
                        window.set_desktop_icon2_x(drop_x);
                        window.set_desktop_icon2_y(drop_y);
                        window.set_show_desktop_tv(true); 
                    }
                }
            } else {
                // Dropped on the dock! Let's reorder!
                let size = window.window().size();
                let w_width = size.width as f32;
                let rel_x = drop_x - w_width / 2.0;
                
                let target_idx = if rel_x < -76.0 { 0 }
                                 else if rel_x < 0.0 { 1 }
                                 else if rel_x < 76.0 { 2 }
                                 else { 3 };
                
                if let Some(icon) = dock_icons_drop.row_data(idx as usize) {
                    dock_icons_drop.remove(idx as usize);
                    let mut insert_idx = target_idx as usize;
                    
                    // If moving forward, adjust for the shift caused by removal
                    if insert_idx > idx as usize {
                        insert_idx -= 1;
                    }
                    
                    if insert_idx > dock_icons_drop.row_count() {
                        insert_idx = dock_icons_drop.row_count();
                    }
                    
                    eprintln!("Taskbar Reorder: idx={}, target_idx={}, insert_idx={}, rel_x={}", idx, target_idx, insert_idx, rel_x);
                    
                    dock_icons_drop.insert(insert_idx, icon);
                    window.set_dock_icons(dock_icons_drop.clone().into());
                }
            }
        }
    });

    let window_weak_desktop_drop = main_window.as_weak();
    let dock_icons_desktop_drop = dock_icons.clone();
    main_window.on_desktop_icon_dropped_in_dock(move |id| {
        if let Some(_window) = window_weak_desktop_drop.upgrade() {
            let mut exists = false;
            for i in 0..dock_icons_desktop_drop.row_count() {
                if let Some(icon) = dock_icons_desktop_drop.row_data(i) {
                    if icon.id == id { exists = true; break; }
                }
            }
            if !exists {
                let new_icon = if id == "files" {
                    AppIcon { id: "files".into(), name: "Files".into(), icon: "📁".into(), bg_color: slint::Color::from_argb_encoded(0xff3b82f6), hover_bg_color: slint::Color::from_argb_encoded(0xff2563eb) }
                } else if id == "console" {
                    AppIcon { id: "console".into(), name: "Console".into(), icon: "💻".into(), bg_color: slint::Color::from_argb_encoded(0xff10b981), hover_bg_color: slint::Color::from_argb_encoded(0xff059669) }
                } else {
                    AppIcon { id: "tv".into(), name: "TV".into(), icon: "📺".into(), bg_color: slint::Color::from_argb_encoded(0xfff97316), hover_bg_color: slint::Color::from_argb_encoded(0xffea580c) }
                };
                dock_icons_desktop_drop.push(new_icon);
            }
        }
    });

    let window_weak = main_window.as_weak();
    let timer_active = Arc::new(Mutex::new(0u32));
    
    let timer_active_clone = timer_active.clone();
    main_window.on_trigger_volume_timer(move || {
        let window_weak = window_weak.clone();
        let timer_active = timer_active_clone.clone();
        
        // Lock mutex and increment session ID
        let mut session = timer_active.lock().unwrap();
        *session += 1;
        let current_session = *session;
        drop(session);
        
        // Spawn background thread to wait and reset show-volume-bar
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            let session = timer_active.lock().unwrap();
            if *session == current_session {
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.upgrade() {
                        window.set_show_volume_bar(false);
                    }
                });
            }
        });
    });
    
    let window_weak_brightness = main_window.as_weak();
    let timer_active_brightness = Arc::new(Mutex::new(0u32));
    let timer_active_brightness_clone = timer_active_brightness.clone();
    main_window.on_trigger_brightness_timer(move || {
        let window_weak = window_weak_brightness.clone();
        let timer_active = timer_active_brightness_clone.clone();
        
        let mut session = timer_active.lock().unwrap();
        *session += 1;
        let current_session = *session;
        drop(session);
        
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            let session = timer_active.lock().unwrap();
            if *session == current_session {
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.upgrade() {
                        window.set_show_brightness_bar(false);
                    }
                });
            }
        });
    });
    
    // FIX #5: Dynamic real-time clock sync timer (runs lightweight system date query)
    let window_weak_time = main_window.as_weak();
    let initial_time = std::process::Command::new("date")
        .arg("+%I:%M %p")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string().to_uppercase())
        .unwrap_or_else(|_| "12:00 PM".to_string());
    main_window.set_current_time(initial_time.into());

    let time_timer = slint::Timer::default();
    time_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(15), move || {
        if let Some(window) = window_weak_time.upgrade() {
            let output = std::process::Command::new("date")
                .arg("+%I:%M %p")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string().to_uppercase())
                .unwrap_or_else(|_| "12:00 PM".to_string());
            window.set_current_time(output.into());
            
            // Real-time battery check
            let bat = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
                .map(|s| format!("🔋 {}%", s.trim()))
                .unwrap_or_else(|_| "🔋 100%".to_string());
            window.set_battery_status(bat.into());

            // Real-time network check
            let net = if std::process::Command::new("ping").arg("-c").arg("1").arg("8.8.8.8").output().map(|o| o.status.success()).unwrap_or(false) {
                "📶"
            } else {
                "⚠️"
            };
            window.set_network_status(net.into());
        }
    });

    // Premium multi-mode Screensaver & Idle Clock synchronization timer
    let window_weak_ss = main_window.as_weak();
    let ss_timer = slint::Timer::default();
    let mut ss_slide_counter = 0;
    ss_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if let Some(window) = window_weak_ss.upgrade() {
            let timeout = window.get_screensaver_timeout();
            if timeout > 0 {
                if window.get_screensaver_active() {
                    ss_slide_counter += 1;
                    if ss_slide_counter >= 4 { // Rotate screensaver images every 4 seconds
                        ss_slide_counter = 0;
                        let next_idx = (window.get_screensaver_index() + 1) % 10;
                        window.set_screensaver_index(next_idx);
                    }
                } else {
                    ss_slide_counter = 0;
                    let idle = window.get_idle_seconds() + 1;
                    window.set_idle_seconds(idle);
                    if idle >= timeout {
                        window.set_screensaver_active(true);
                        window.set_screensaver_index(0);
                    }
                }
            } else {
                ss_slide_counter = 0;
            }
        }
    });
    
    // Asynchronous real-time Wi-Fi connection simulation driven by the Rust backend
    let window_weak_wifi = main_window.as_weak();
    main_window.on_connect_to_wifi(move |net, _pwd| {
        let window_weak = window_weak_wifi.clone();
        let net = net.to_string();
        
        if let Some(window) = window_weak.upgrade() {
            window.set_wifi_connecting(true);
            window.set_wifi_connection_status(format!("جاري التحقق من كلمة المرور لـ {}...", net).into());
        }

        thread::spawn(move || {
            // Stage 1: Authenticating/Checking credentials
            thread::sleep(Duration::from_millis(1500));
            let window_weak_s1 = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak_s1.upgrade() {
                    window.set_wifi_connection_status("تم قبول كلمة المرور! جاري الحصول على عنوان IP...".into());
                }
            });

            // Stage 2: Connection Successful!
            thread::sleep(Duration::from_millis(1500));
            let window_weak_s2 = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak_s2.upgrade() {
                    window.set_wifi_connection_status("تم الاتصال بنجاح! 🟢".into());
                    window.set_wifi_enabled(true);
                }
            });

            // Stage 3: Close connection screens and reset connecting states
            thread::sleep(Duration::from_millis(1200));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    window.set_wifi_connecting(false);
                    window.set_show_password_screen(false);
                    window.set_show_settings(false);
                }
            });
        });
    });
    // Asynchronous real-time Bluetooth pairing simulation
    let window_weak_bt = main_window.as_weak();
    main_window.on_bluetooth_device_clicked(move |dev| {
        let window_weak = window_weak_bt.clone();
        let dev = dev.to_string();
        
        if let Some(window) = window_weak.upgrade() {
            window.set_bluetooth_connecting(true);
            window.set_bluetooth_connection_status(format!("جاري الاقتران مع {}...", dev).into());
        }

        thread::spawn(move || {
            // Stage 1: Connecting / pairing
            thread::sleep(Duration::from_millis(1500));
            let window_weak_s1 = window_weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak_s1.upgrade() {
                    window.set_bluetooth_connection_status("تم الاقتران بنجاح! 🔵".into());
                }
            });

            // Stage 2: Clean up and close setting sidebar
            thread::sleep(Duration::from_millis(1000));
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    window.set_bluetooth_connecting(false);
                    window.set_show_settings(false);
                }
            });
        });
    });
    
    // Real-time integration of tigris-shell CLI with the Sumer UI terminal
    main_window.on_execute_command(move |cmd| {
        let cmd = cmd.trim().to_string();
        if cmd.is_empty() {
            return "".into();
        }

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let main_cmd = parts[0];
        let args = &parts[1..];

        // 1. Direct Rust implementations for ultra-speed/portable fallback
        match main_cmd {
            "help" | "مساعدة" => {
                return format!(
                    "شيل دجلة - أداة النظام الموحدة\nالأوامر المتوفرة:\n  help / مساعدة    - عرض هذه المساعدة\n  about / حول      - معلومات حول دجلة\n  sysinfo / نظام   - معلومات نظام التشغيل\n  neofetch / شعار  - شعار النظام ومعلومات معالجة\n  wifi / وايفاي    - التحكم بالواي فاي [on / off / status]\n  airplane / طيران - التحكم بوضع الطيران [on / off / status]\n  battery / بطارية - فحص حالة البطارية\n  network / شبكة   - فحص الاتصال بالإنترنت"
                ).into();
            }
            "about" | "حول" => {
                return "Tigris: شيل دجلة هو محرك العتاد والأوامر التفاعلي لنظام Sumer OS.\nتمت إعادة تصميمه بالكامل بلغة Rust الصرفة ليكون فائق الخفة والسرعة.".into();
            }
            _ => {}
        }

        // 2. Call the compiled `tigris-shell` executable!
        // We look for `/usr/bin/tigris-shell` first, then local `../programs/tigris-shell/target/release/tigris-shell`,
        // then fallback to standard command execution.
        let mut path = "/usr/bin/tigris-shell".to_string();
        if !std::path::Path::new(&path).exists() {
            let debug_paths = vec![
                "../programs/tigris-shell/target/debug/tigris-shell".to_string(),
                "../tigris-shell/target/debug/tigris-shell".to_string(),
            ];
            for dp in debug_paths {
                if std::path::Path::new(&dp).exists() {
                    path = dp;
                    break;
                }
            }
        }
        // Also fallback to cargo run in tigris-shell if not compiled yet!
        if !std::path::Path::new(&path).exists() {
            let manifest_path = if std::path::Path::new("../programs/tigris-shell/Cargo.toml").exists() {
                "../programs/tigris-shell/Cargo.toml"
            } else {
                "../tigris-shell/Cargo.toml"
            };
            
            // Run cargo run --manifest-path <manifest> -- <args>
            let mut cargo_args = vec!["run", "--manifest-path", manifest_path, "--"];
            cargo_args.push(main_cmd);
            for arg in args {
                cargo_args.push(arg);
            }
            if let Ok(output) = std::process::Command::new("cargo")
                .args(&cargo_args)
                .output()
            {
                if output.status.success() {
                    return String::from_utf8_lossy(&output.stdout).into_owned().into();
                }
            }
            
            // Fallback directly to native terminal execution of system commands (like ls, pwd) if tigris-shell is not found
            if let Ok(output) = std::process::Command::new(main_cmd).args(args).output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if output.status.success() {
                    return stdout.into_owned().into();
                } else {
                    return format!("{}\n{}", stderr, stdout).into();
                }
            }
            return format!("خطأ: تعذر تشغيل الأمر '{}' في النظام المضيف.", main_cmd).into();
        }

        // Execute tigris-shell binary directly
        let mut tigris_args = vec![main_cmd];
        for arg in args {
            tigris_args.push(arg);
        }

        match std::process::Command::new(&path).args(&tigris_args).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if output.status.success() {
                    stdout.into_owned().into()
                } else {
                    format!("{}\n{}", stderr, stdout).into()
                }
            }
            Err(_) => {
                format!("خطأ: فشل في تشغيل شيل دجلة في المسار: {}", path).into()
            }
        }
    });
    
    // Play navigation sound handler for Nahr TV
    main_window.on_play_navigation_sound(move || {
        thread::spawn(move || {
            let paths = vec![
                "../nahr-tv/ui/firmament/assets/audio.mp3",
                "../programs/nahr-tv/ui/firmament/assets/audio.mp3",
                "ui/firmament/assets/audio.mp3",
                "/usr/bin/firmament/assets/audio.mp3",
                "/usr/share/sumer/audio.mp3",
            ];
            let mut sound_path = "../nahr-tv/ui/firmament/assets/audio.mp3".to_string();
            for p in &paths {
                if std::path::Path::new(p).exists() {
                    sound_path = p.to_string();
                    break;
                }
            }
            // Spawn command asynchronously to keep latency extremely low
            if std::process::Command::new("gst-play-1.0").arg(&sound_path).spawn().is_ok() { return; }
            if std::process::Command::new("mpg123").arg("-q").arg(&sound_path).spawn().is_ok() { return; }
            if std::process::Command::new("pw-play").arg(&sound_path).spawn().is_ok() { return; }
            if std::process::Command::new("paplay").arg(&sound_path).spawn().is_ok() { return; }
            if std::process::Command::new("ffplay").args(&["-nodisp", "-autoexit", "-loglevel", "quiet", &sound_path]).spawn().is_ok() { return; }
        });
    });
    
    main_window.run()
}

