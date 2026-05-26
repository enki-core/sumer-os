// استوديو بريزم المتقدم للقطات الشاشة - نظام سومر OS
// Prism Screenshot Rust Backend
// Coordinates global hotkeys, instant capturing, canvas painting, gradient mockups, and X11/Wayland clipboard.

slint::include_modules!();

mod clipboard;
mod binary_loader;
mod integration;

use slint::{ComponentHandle, Image, SharedPixelBuffer};
use screenshots::Screen;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;

// Helper to convert RgbaImage to Slint Image
fn to_slint_image(img: &image::RgbaImage) -> Image {
    let buffer = SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        img.as_raw(),
        img.width(),
        img.height(),
    );
    Image::from_rgba8(buffer)
}

// Global drawing stroke helpers
fn draw_thick_line(img: &mut image::RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: image::Rgba<u8>, thickness: i32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut cx = x0;
    let mut cy = y0;

    loop {
        for dy_t in -thickness..=thickness {
            for dx_t in -thickness..=thickness {
                if dx_t * dx_t + dy_t * dy_t <= thickness * thickness {
                    let px = cx + dx_t;
                    let py = cy + dy_t;
                    if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                        img.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }

        if cx == x1 && cy == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            cx += sx;
        }
        if e2 < dx {
            err += dx;
            cy += sy;
        }
    }
}

fn draw_rect(img: &mut image::RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: image::Rgba<u8>, thickness: i32) {
    let min_x = x0.min(x1);
    let max_x = x0.max(x1);
    let min_y = y0.min(y1);
    let max_y = y0.max(y1);

    // Horizontal borders
    for x in min_x..=max_x {
        for t in -thickness/2..=thickness/2 {
            let py1 = min_y + t;
            let py2 = max_y + t;
            if x >= 0 && x < img.width() as i32 {
                if py1 >= 0 && py1 < img.height() as i32 {
                    img.put_pixel(x as u32, py1 as u32, color);
                }
                if py2 >= 0 && py2 < img.height() as i32 {
                    img.put_pixel(x as u32, py2 as u32, color);
                }
            }
        }
    }

    // Vertical borders
    for y in min_y..=max_y {
        for t in -thickness/2..=thickness/2 {
            let px1 = min_x + t;
            let px2 = max_x + t;
            if y >= 0 && y < img.height() as i32 {
                if px1 >= 0 && px1 < img.width() as i32 {
                    img.put_pixel(px1 as u32, y as u32, color);
                }
                if px2 >= 0 && px2 < img.width() as i32 {
                    img.put_pixel(px2 as u32, y as u32, color);
                }
            }
        }
    }
}

fn draw_arrow(img: &mut image::RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: image::Rgba<u8>, thickness: i32) {
    // Shaft
    draw_thick_line(img, x0, y0, x1, y1, color, thickness);

    // Arrowhead
    let angle = ((y1 - y0) as f32).atan2((x1 - x0) as f32);
    let arrow_len = 25.0f32; 
    let arrow_angle = 0.45; 

    let x_branch1 = x1 as f32 - arrow_len * (angle - arrow_angle).cos();
    let y_branch1 = y1 as f32 - arrow_len * (angle - arrow_angle).sin();

    let x_branch2 = x1 as f32 - arrow_len * (angle + arrow_angle).cos();
    let y_branch2 = y1 as f32 - arrow_len * (angle + arrow_angle).sin();

    draw_thick_line(img, x1, y1, x_branch1 as i32, y_branch1 as i32, color, thickness);
    draw_thick_line(img, x1, y1, x_branch2 as i32, y_branch2 as i32, color, thickness);
}

// Breathtaking Studio Beautifier Composite Generator
fn render_mockup(
    screenshot: &image::RgbaImage,
    backdrop_style: i32,
    padding: f32,
    corner_radius: f32,
    show_macos_bar: bool,
    shadow_style: i32,
) -> image::RgbaImage {
    let canvas_w = 1200;
    let canvas_h = 800;

    let mut canvas = image::RgbaImage::new(canvas_w, canvas_h);

    // 1. Draw Gradient Background
    for y in 0..canvas_h {
        for x in 0..canvas_w {
            let t_x = x as f32 / canvas_w as f32;
            let t_y = y as f32 / canvas_h as f32;
            
            let col = match backdrop_style {
                0 => { // Sunset: Orange (#f97316) -> Pink (#ec4899) -> Purple (#8b5cf6)
                    let r = (249.0 * (1.0 - t_x) + 236.0 * t_x * (1.0 - t_y) + 139.0 * t_y) as u8;
                    let g = (115.0 * (1.0 - t_x) + 72.0 * t_x * (1.0 - t_y) + 92.0 * t_y) as u8;
                    let b = (22.0 * (1.0 - t_x) + 153.0 * t_x * (1.0 - t_y) + 246.0 * t_y) as u8;
                    image::Rgba([r, g, b, 255])
                }
                1 => { // Ocean: Cyan (#0ea5e9) -> Blue (#3b82f6) -> Indigo (#4f46e5)
                    let r = (14.0 * (1.0 - t_x) + 59.0 * t_x * (1.0 - t_y) + 79.0 * t_y) as u8;
                    let g = (165.0 * (1.0 - t_x) + 130.0 * t_x * (1.0 - t_y) + 70.0 * t_y) as u8;
                    let b = (233.0 * (1.0 - t_x) + 246.0 * t_x * (1.0 - t_y) + 229.0 * t_y) as u8;
                    image::Rgba([r, g, b, 255])
                }
                2 => { // Emerald: Mint (#10b981) -> Teal (#14b8a6) -> Cyan (#06b6d4)
                    let r = (16.0 * (1.0 - t_x) + 20.0 * t_x * (1.0 - t_y) + 6.0 * t_y) as u8;
                    let g = (185.0 * (1.0 - t_x) + 184.0 * t_x * (1.0 - t_y) + 182.0 * t_y) as u8;
                    let b = (129.0 * (1.0 - t_x) + 166.0 * t_x * (1.0 - t_y) + 212.0 * t_y) as u8;
                    image::Rgba([r, g, b, 255])
                }
                3 => { // Aurora: Purple (#a855f7) -> Pink (#d946ef) -> Rose (#f43f5e)
                    let r = (168.0 * (1.0 - t_x) + 217.0 * t_x * (1.0 - t_y) + 244.0 * t_y) as u8;
                    let g = (85.0 * (1.0 - t_x) + 70.0 * t_x * (1.0 - t_y) + 63.0 * t_y) as u8;
                    let b = (247.0 * (1.0 - t_x) + 239.0 * t_x * (1.0 - t_y) + 94.0 * t_y) as u8;
                    image::Rgba([r, g, b, 255])
                }
                4 => { // Mystic Dark: Charcoal -> Slate
                    let r = (30.0 * (1.0 - t_y) + 15.0 * t_y) as u8;
                    let g = (41.0 * (1.0 - t_y) + 23.0 * t_y) as u8;
                    let b = (59.0 * (1.0 - t_y) + 42.0 * t_y) as u8;
                    image::Rgba([r, g, b, 255])
                }
                5 => { // Flat Dark
                    image::Rgba([2, 6, 23, 255])
                }
                _ => { // Transparent (for exporting raw mockups)
                    image::Rgba([0, 0, 0, 0])
                }
            };
            canvas.put_pixel(x, y, col);
        }
    }

    // 2. Mockup dimensions
    let pad_w = (canvas_w as f32 * padding / 200.0) as u32;
    let pad_h = (canvas_h as f32 * padding / 200.0) as u32;

    let inner_w = canvas_w - pad_w * 2;
    let inner_h = canvas_h - pad_h * 2;

    if inner_w < 50 || inner_h < 50 {
        return canvas;
    }

    // Resize screenshot
    let resized_screenshot = image::imageops::resize(
        screenshot,
        inner_w,
        inner_h,
        image::imageops::FilterType::Lanczos3,
    );

    // 3. Draw high-fidelity neon shadow
    if shadow_style > 0 {
        let shadow_color = match shadow_style {
            1 => image::Rgba([0, 0, 0, 160]),
            _ => image::Rgba([99, 102, 241, 130]),
        };

        let shadow_margin = 25;
        for sy in (pad_h as i32 - shadow_margin)..=(pad_h as i32 + inner_h as i32 + shadow_margin) {
            for sx in (pad_w as i32 - shadow_margin)..=(pad_w as i32 + inner_w as i32 + shadow_margin) {
                if sx >= 0 && sx < canvas_w as i32 && sy >= 0 && sy < canvas_h as i32 {
                    let dx = if sx < pad_w as i32 { pad_w as i32 - sx } else if sx > (pad_w + inner_w) as i32 { sx - (pad_w + inner_w) as i32 } else { 0 };
                    let dy = if sy < pad_h as i32 { pad_h as i32 - sy } else if sy > (pad_h + inner_h) as i32 { sy - (pad_h + inner_h) as i32 } else { 0 };
                    
                    let dist = ((dx * dx + dy * dy) as f32).sqrt();
                    if dist <= shadow_margin as f32 {
                        let alpha = (1.0 - dist / shadow_margin as f32) * (shadow_color[3] as f32 / 255.0);
                        let bg = canvas.get_pixel(sx as u32, sy as u32);
                        
                        let blended = image::Rgba([
                            (bg[0] as f32 * (1.0 - alpha) + shadow_color[0] as f32 * alpha) as u8,
                            (bg[1] as f32 * (1.0 - alpha) + shadow_color[1] as f32 * alpha) as u8,
                            (bg[2] as f32 * (1.0 - alpha) + shadow_color[2] as f32 * alpha) as u8,
                            255,
                        ]);
                        canvas.put_pixel(sx as u32, sy as u32, blended);
                    }
                }
            }
        }
    }

    // 4. Draw rounded corner clip
    let is_inside_rounded = |cx: u32, cy: u32, w: u32, h: u32, rad: f32| -> bool {
        let x = cx as f32;
        let y = cy as f32;
        let wf = w as f32;
        let hf = h as f32;

        if x < rad && y < rad {
            (x - rad).powi(2) + (y - rad).powi(2) <= rad.powi(2)
        } else if x > wf - rad && y < rad {
            (x - (wf - rad)).powi(2) + (y - rad).powi(2) <= rad.powi(2)
        } else if x < rad && y > hf - rad {
            (x - rad).powi(2) + (y - (hf - rad)).powi(2) <= rad.powi(2)
        } else if x > wf - rad && y > hf - rad {
            (x - (wf - rad)).powi(2) + (y - (hf - rad)).powi(2) <= rad.powi(2)
        } else {
            true
        }
    };

    for y in 0..inner_h {
        for x in 0..inner_w {
            let cx = pad_w + x;
            let cy = pad_h + y;

            if is_inside_rounded(x, y, inner_w, inner_h, corner_radius) {
                let pixel = resized_screenshot.get_pixel(x, y);
                canvas.put_pixel(cx, cy, *pixel);
            }
        }
    }

    // 5. macOS Window Controls Dots
    if show_macos_bar {
        let bar_height = 24;
        let dot_r = 5;
        let dot_y = pad_h + 16;
        let colors = [
            image::Rgba([255, 95, 86, 255]),
            image::Rgba([255, 189, 46, 255]),
            image::Rgba([39, 201, 63, 255]),
        ];

        for (idx, col) in colors.iter().enumerate() {
            let dot_x = pad_w + 24 + (idx as u32 * 20);
            for dy in -dot_r..=dot_r {
                for dx in -dot_r..=dot_r {
                    if dx * dx + dy * dy <= dot_r * dot_r {
                        let px = dot_x as i32 + dx;
                        let py = dot_y as i32 + dy;
                        if px >= 0 && px < canvas_w as i32 && py >= 0 && py < canvas_h as i32 {
                            canvas.put_pixel(px as u32, py as u32, *col);
                        }
                    }
                }
            }
        }
    }

    canvas
}

// App context struct to hold screenshots state and history list
struct ScreenshotState {
    raw_screenshot: Option<image::RgbaImage>,
    annotated_screenshot: Option<image::RgbaImage>,
    drawing_layer: image::RgbaImage,
    history: Vec<image::RgbaImage>,
    active_history_id: usize,
}

fn main() -> Result<(), slint::PlatformError> {
    // 📦 تحميل معلومات البرنامج الثنائي
    println!("\n🎯 جاري بدء تشغيل Prism-Screenshot...");
    match binary_loader::BinaryLoader::new() {
        Ok(loader) => {
            // تحقق من صحة الملف الثنائي
            match loader.validate() {
                Ok(_) => {
                    // اطبع معلومات البرنامج
                    if let Ok(info) = loader.get_binary_info() {
                        info.print();
                    }
                    println!("✅ البرنامج الثنائي صحيح ومستعد للتشغيل");
                },
                Err(e) => {
                    eprintln!("⚠️ تحذير: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("❌ خطأ: {}", e);
            eprintln!("⚠️ سيتم محاولة المتابعة بأي حال...");
        }
    }
    
    // 🔗 إنشاء نقطة التكامل مع Ur-Desktop
    match integration::PrismIntegration::new() {
        Ok(integration) => {
            println!("✅ تم إنشاء نقطة التكامل مع Ur-Desktop");
            let _ = integration.notify_desktop_starting();
        },
        Err(e) => eprintln!("⚠️ تحذير التكامل: {}", e),
    }
    
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();

    // Context wrap
    let state = Arc::new(Mutex::new(ScreenshotState {
        raw_screenshot: None,
        annotated_screenshot: None,
        drawing_layer: image::RgbaImage::new(100, 100), // resized dynamically
        history: Vec::new(),
        active_history_id: 0,
    }));

    // Start background PrintScreen global hotkey daemon!
    let app_weak_kbd = app.as_weak();
    thread::spawn(move || {
        // We set up rdev global event listening. If it is permitted, it will trigger nicely on PrintScreen!
        let callback = move |event: rdev::Event| {
            if let rdev::EventType::KeyPress(key) = event.event_type {
                // Check if PrintScreen or F12 key (convenient fallback!) is pressed
                if key == rdev::Key::PrintScreen || key == rdev::Key::F12 {
                    let app_weak_clone = app_weak_kbd.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_clone.upgrade() {
                            // If running in dock bar mode, activate the capture right away!
                            app.invoke_trigger_capture();
                        }
                    });
                }
            }
        };

        let _ = rdev::listen(callback);
    });

    // Helper to refresh Slint GUI with latest beautifications
    let state_clone = state.clone();
    let app_weak_clone = app.as_weak();
    let update_preview = move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let mut st = state_clone.lock().unwrap();
            
            // Compose raw + drawing layer
            if let Some(raw) = &st.raw_screenshot {
                let mut composite = raw.clone();
                
                // Overlay drawing layer
                if st.drawing_layer.width() == raw.width() && st.drawing_layer.height() == raw.height() {
                    for y in 0..raw.height() {
                        for x in 0..raw.width() {
                            let draw_pixel = st.drawing_layer.get_pixel(x, y);
                            if draw_pixel[3] > 0 {
                                composite.put_pixel(x, y, *draw_pixel);
                            }
                        }
                    }
                }
                
                st.annotated_screenshot = Some(composite.clone());

                // Apply professional studio beautification gradients
                let beautified = render_mockup(
                    &composite,
                    app.get_backdrop_style(),
                    app.get_padding_size(),
                    app.get_corner_radius(),
                    app.get_show_macos_bar(),
                    app.get_shadow_style(),
                );

                // Update processed-preview Slint property
                app.set_processed_preview(to_slint_image(&beautified));
            }
        }
    };

    // Callback 1: render-beautified
    let update_preview_clone = update_preview.clone();
    app.on_render_beautified(update_preview_clone);

    // Callback 2: backdrop-changed
    let update_preview_clone = update_preview.clone();
    app.on_backdrop_changed(move |style| {
        update_preview_clone();
    });

    // Callback 3: draw-paint-stroke (draws line, rect, or arrow directly onto raw screenshot canvas)
    let state_paint = state.clone();
    let update_preview_paint = update_preview.clone();
    app.on_draw_paint_stroke(move |x1, y1, x2, y2, tool, color_idx, size| {
        let mut st = state_paint.lock().unwrap();
        if let Some(raw) = &st.raw_screenshot {
            // Guarantee drawing layer matches raw screen resolution
            if st.drawing_layer.width() != raw.width() || st.drawing_layer.height() != raw.height() {
                st.drawing_layer = image::RgbaImage::new(raw.width(), raw.height());
            }

            let w = raw.width() as i32;
            let h = raw.height() as i32;

            let px1 = (x1 * w as f32) as i32;
            let py1 = (y1 * h as f32) as i32;
            let px2 = (x2 * w as f32) as i32;
            let py2 = (y2 * h as f32) as i32;

            let colors = [
                image::Rgba([239, 68, 68, 255]),  // Red
                image::Rgba([234, 179, 8, 255]),  // Yellow
                image::Rgba([34, 197, 94, 255]),  // Green
                image::Rgba([59, 130, 246, 255]), // Blue
                image::Rgba([255, 255, 255, 255]), // White
            ];
            let color = colors[color_idx as usize % 5];

            match tool {
                0 => { // Pen
                    draw_thick_line(&mut st.drawing_layer, px1, py1, px2, py2, color, size as i32);
                }
                1 => { // Rectangle
                    draw_rect(&mut st.drawing_layer, px1, py1, px2, py2, color, size as i32);
                }
                2 => { // Arrow
                    draw_arrow(&mut st.drawing_layer, px1, py1, px2, py2, color, size as i32);
                }
                _ => {} // Eraser (not implemented yet, clear annotations covers it)
            }
        }
        drop(st);
        update_preview_paint();
    });

    // Callback 4: clear-annotations
    let state_clear = state.clone();
    let update_preview_clear = update_preview.clone();
    app.on_clear_annotations(move || {
        let mut st = state_clear.lock().unwrap();
        if let Some(raw) = &st.raw_screenshot {
            st.drawing_layer = image::RgbaImage::new(raw.width(), raw.height());
        }
        drop(st);
        update_preview_clear();
    });

    // Callback 5: trigger-capture (performs full screen capture after delay countdown if enabled)
    let state_capture = state.clone();
    let app_weak_capture = app.as_weak();
    let update_preview_capture = update_preview.clone();
    app.on_trigger_capture(move || {
        if let Some(app) = app_weak_capture.upgrade() {
            let delay_idx = app.get_capture_delay();
            let delay_seconds = match delay_idx {
                0 => 0,
                1 => 3,
                2 => 5,
                _ => 10,
            };

            let app_clone = app_weak_capture.clone();
            let state_clone = state_capture.clone();
            let update_preview_clone = update_preview_capture.clone();

            thread::spawn(move || {
                if delay_seconds > 0 {
                    // Show countdown window
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_clone.upgrade() {
                            app.set_show_countdown(true);
                            app.set_countdown_number(delay_seconds);
                        }
                    });

                    // Countdown ticking
                    for i in (1..=delay_seconds).rev() {
                        thread::sleep(Duration::from_secs(1));
                        let app_clone_tick = app_clone.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = app_clone_tick.upgrade() {
                                app.set_countdown_number(i - 1);
                            }
                        });
                    }
                }

                // Hide main app window so it doesn't appear in the screenshot!
                let app_clone_hide = app_clone.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone_hide.upgrade() {
                        app.window().hide().unwrap();
                    }
                });

                // Wait 400ms for window to completely fade/close in X11
                thread::sleep(Duration::from_millis(400));

                // Capture using screenshots crate!
                let screens = Screen::all().unwrap_or_default();
                let captured_image = if let Some(screen) = screens.first() {
                    if let Ok(img) = screen.capture() {
                        // Convert screenshots::Image to image::RgbaImage
                        image::RgbaImage::from_raw(img.width(), img.height(), img.rgba().to_vec())
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Restore/Show window and load captured screenshot
                let app_clone_restore = app_clone.clone();
                let state_clone_restore = state_clone.clone();
                let update_preview_clone_restore = update_preview_clone.clone();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone_restore.upgrade() {
                        app.set_show_countdown(false);
                        app.window().show().unwrap();

                        if let Some(img) = captured_image {
                            let mut st = state_clone_restore.lock().unwrap();
                            st.raw_screenshot = Some(img.clone());
                            st.drawing_layer = image::RgbaImage::new(img.width(), img.height());
                            
                            // Add to history
                            st.history.push(img.clone());
                            st.active_history_id = st.history.len() - 1;

                            // Update Slint history items
                            let mut history_model = Vec::new();
                            for (idx, hist_img) in st.history.iter().enumerate() {
                                // Create miniature thumbnail to save memory
                                let thumb = image::imageops::resize(
                                    hist_img,
                                    120,
                                    80,
                                    image::imageops::FilterType::Nearest,
                                );
                                history_model.push(HistoryItem {
                                    id: idx as i32,
                                    thumbnail: to_slint_image(&thumb),
                                    is_active: idx == st.active_history_id,
                                });
                            }
                            app.set_history_list(slint::ModelRc::new(slint::VecModel::from(history_model)));
                            drop(st);

                            // Enter professional Studio mode automatically upon taking screenshot!
                            app.set_is_studio-mode(true);
                            app.set_status_message("تم التقاط لقطة الشاشة بنجاح!".into());

                            // Trigger auto-copy to clipboard if enabled
                            if app.get_auto_copy_clipboard() {
                                if clipboard::copy_image(&img).is_ok() {
                                    app.set_status_message("تم التقاط اللقطة ونسخها للحافظة!".into());
                                }
                            }

                            update_preview_clone_restore();
                        } else {
                            app.set_status_message("فشل التقاط لقطة الشاشة!".into());
                        }
                    }
                });
            });
        }
    });

    // Callback 6: trigger-area-capture (captures screen and lets them edit/beautify it)
    let app_weak_area = app.as_weak();
    app.on_trigger_area-capture(move || {
        if let Some(app) = app_weak_area.upgrade() {
            // In Area capture, we capture full screen first, and auto-navigate to Studio 
            // where they can utilize standard crop, annotations, and mockups padding!
            app.invoke_trigger_capture();
        }
    });

    // Callback 7: select-gallery-item (allows reloading any previous screenshot from thumbnail strip)
    let state_gal = state.clone();
    let update_preview_gal = update_preview.clone();
    let app_weak_gal = app.as_weak();
    app.on_select_gallery_item(move |idx| {
        if let Some(app) = app_weak_gal.upgrade() {
            let mut st = state_gal.lock().unwrap();
            let idx_u = idx as usize;
            if idx_u < st.history.len() {
                st.active_history_id = idx_u;
                let active_img = st.history[idx_u].clone();
                st.raw_screenshot = Some(active_img.clone());
                st.drawing_layer = image::RgbaImage::new(active_img.width(), active_img.height());

                // Update active highlight
                let mut history_model = Vec::new();
                for (i, hist_img) in st.history.iter().enumerate() {
                    let thumb = image::imageops::resize(
                        hist_img,
                        120,
                        80,
                        image::imageops::FilterType::Nearest,
                    );
                    history_model.push(HistoryItem {
                        id: i as i32,
                        thumbnail: to_slint_image(&thumb),
                        is_active: i == idx_u,
                    });
                }
                app.set_history_list(slint::ModelRc::new(slint::VecModel::from(history_model)));
                app.set_status_message(format!("تم تحميل اللقطة السابقة رقم {}", idx + 1).into());
            }
            drop(st);
            update_preview_gal();
        }
    });

    // Callback 8: copy-image-to-clipboard (copies the beautiful mockup presentation directly!)
    let state_copy = state.clone();
    let app_weak_copy = app.as_weak();
    app.on_copy_image_to_clipboard(move || {
        if let Some(app) = app_weak_copy.upgrade() {
            let st = state_copy.lock().unwrap();
            if let Some(composite) = &st.annotated_screenshot {
                // Render the mockup presentation card for copying!
                let beautified = render_mockup(
                    composite,
                    app.get_backdrop_style(),
                    app.get_padding_size(),
                    app.get_corner_radius(),
                    app.get_show_macos_bar(),
                    app.get_shadow_style(),
                );

                match clipboard::copy_image(&beautified) {
                    Ok(_) => app.set_status_message("تم نسخ لقطة الشاشة المزينة والجاهزة للعرض إلى الحافظة!".into()),
                    Err(e) => app.set_status_message(format!("خطأ في النسخ: {}", e).into()),
                }
            } else {
                app.set_status_message("لا توجد لقطة شاشة لنسخها!".into());
            }
        }
    });

    // Callback 9: save-image (saves png to user-selected file dialogue)
    let state_save = state.clone();
    let app_weak_save = app.as_weak();
    app.on_save_image(move || {
        if let Some(app) = app_weak_save.upgrade() {
            let st = state_save.lock().unwrap();
            if let Some(composite) = &st.annotated_screenshot {
                // Render beautified presentation card
                let beautified = render_mockup(
                    composite,
                    app.get_backdrop_style(),
                    app.get_padding_size(),
                    app.get_corner_radius(),
                    app.get_show_macos_bar(),
                    app.get_shadow_style(),
                );

                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .set_file_name(&format!("prism_screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S")))
                    .save_file()
                {
                    match beautified.save(&path) {
                        Ok(_) => app.set_status_message(format!("تم حفظ الصورة بنجاح في {}", path.display()).into()),
                        Err(e) => app.set_status_message(format!("خطأ في الحفظ: {}", e).into()),
                    }
                }
            } else {
                app.set_status_message("لا توجد لقطة شاشة لحفظها!".into());
            }
        }
    });

    // Callback 10: run-ocr (scans screenshot for text, uses local tesseract with simulated fallback)
    let state_ocr = state.clone();
    let app_weak_ocr = app.as_weak();
    app.on_run_ocr(move || {
        if let Some(app) = app_weak_ocr.upgrade() {
            let st = state_ocr.lock().unwrap();
            if let Some(raw) = &st.raw_screenshot {
                app.set_show_ocr_panel(true);
                app.set_ocr_loading(true);

                // Run in background thread to keep UI completely responsive!
                let raw_clone = raw.clone();
                let app_weak_ocr_thread = app_weak_ocr.clone();
                thread::spawn(move || {
                    // Try to save image temporarily
                    let temp_dir = std::env::temp_dir();
                    let temp_path = temp_dir.join("prism_screenshot_ocr_temp.png");
                    
                    let mut text_extracted = String::new();
                    
                    if raw_clone.save(&temp_path).is_ok() {
                        // Check if tesseract OCR is available in the shell
                        let output = std::process::Command::new("tesseract")
                            .arg(temp_path.to_string_lossy().to_string())
                            .arg("stdout")
                            .output();

                        match output {
                            Ok(out) => {
                                if out.status.success() {
                                    text_extracted = String::from_utf8_lossy(&out.stdout).to_string();
                                }
                            }
                            Err(_) => {}
                        }
                    }

                    // Fallback to beautiful mockup text if local tesseract isn't installed
                    if text_extracted.trim().is_empty() {
                        text_extracted = "⚠️ لم يتم العثور على أداة Tesseract OCR في النظام.\nالرجاء تشغيل 'apk add tesseract-ocr' أو 'apt install tesseract-ocr' لتفعيل خاصية استخراج النصوص الفورية!\n\nولكن إليك نموذجاً افتراضياً للنص المحلل:\n-------------------------------------------------\nالعراق بلاد الرافدين، موطن الحضارة السومرية.\nولد الحرف الأول والتدوين في مدينة أور السومرية التاريخية في عام 3200 قبل الميلاد، لتبدأ من هنا مسيرة الإنسانية مع العلم والأدب والقوانين المنظمة.\n\nتطبيق بريزم Prism Screenshot - نظام Sumer OS 2026\nأداة لقطات الشاشة الاحترافية المدمجة بالكامل.".to_string();
                    }

                    // Load text back in event loop
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_ocr_thread.upgrade() {
                            app.set_ocr_loading(false);
                            app.set_ocr_text(text_extracted.into());
                        }
                    });
                });
            } else {
                app.set_status_message("لا توجد لقطة شاشة للتحليل!".into());
            }
        }
    });

    // Callback 11: close-app
    let app_weak_close = app.as_weak();
    app.on_close_app(move || {
        if let Some(app) = app_weak_close.upgrade() {
            app.window().hide().unwrap();
            std::process::exit(0);
        }
    });

    app.run()
}
