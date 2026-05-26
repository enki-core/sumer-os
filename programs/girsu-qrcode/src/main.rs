// Girsu Advanced QR Code Studio Rust Backend
// Integrates Slint UI with qrcode generation, advanced styling, custom shapes, and export systems.

slint::include_modules!();

use qrcode::EcLevel;
use std::fs::File;
use std::io::Write;

fn parse_hex_color(hex: &str) -> Option<image::Rgba<u8>> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(image::Rgba([r, g, b, 255]))
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(image::Rgba([r, g, b, a]))
    } else {
        None
    }
}

fn in_rounded_rect(cx: f32, cy: f32, w: f32, h: f32, r: f32) -> bool {
    if cx < r && cy < r {
        (cx - r).powi(2) + (cy - r).powi(2) <= r.powi(2)
    } else if cx > w - r && cy < r {
        (cx - (w - r)).powi(2) + (cy - r).powi(2) <= r.powi(2)
    } else if cx < r && cy > h - r {
        (cx - r).powi(2) + (cy - (h - r)).powi(2) <= r.powi(2)
    } else if cx > w - r && cy > h - r {
        (cx - (w - r)).powi(2) + (cy - (h - r)).powi(2) <= r.powi(2)
    } else {
        true
    }
}

fn interpolate_color(
    c1: image::Rgba<u8>,
    c2: image::Rgba<u8>,
    px: f32,
    py: f32,
    img_size: f32,
    style: i32,
) -> image::Rgba<u8> {
    let t = match style {
        0 => 0.0,            // Solid
        1 => px / img_size,  // Linear H
        2 => py / img_size,  // Linear V
        _ => {               // Radial
            let cx = img_size / 2.0;
            let cy = img_size / 2.0;
            let dist = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
            let max_dist = (cx.powi(2) + cy.powi(2)).sqrt();
            dist / max_dist
        }
    };
    let t = t.clamp(0.0, 1.0);
    image::Rgba([
        (c1[0] as f32 * (1.0 - t) + c2[0] as f32 * t) as u8,
        (c1[1] as f32 * (1.0 - t) + c2[1] as f32 * t) as u8,
        (c1[2] as f32 * (1.0 - t) + c2[2] as f32 * t) as u8,
        (c1[3] as f32 * (1.0 - t) + c2[3] as f32 * t) as u8,
    ])
}

fn copy_png_to_clipboard_linux(png_path: &str) -> Result<(), String> {
    // Check if xclip is installed and works
    let child = std::process::Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .arg("-t")
        .arg("image/png")
        .arg("-i")
        .arg(png_path)
        .spawn();

    if let Ok(mut child) = child {
        let _ = child.wait();
        return Ok(());
    }

    // Alternatively try wl-copy for Wayland
    let child = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("wl-copy < {}", png_path))
        .spawn();

    if let Ok(mut child) = child {
        let _ = child.wait();
        return Ok(());
    }

    Err("Neither xclip nor wl-copy are available. Please install xclip or wl-clipboard.".to_string())
}

fn copy_text_to_clipboard_linux(text: &str) -> Result<(), String> {
    // Try xclip
    let child = std::process::Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn();

    if let Ok(mut child) = child {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        return Ok(());
    }

    // Try wl-copy
    let child = std::process::Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn();

    if let Ok(mut child) = child {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
        return Ok(());
    }

    Err("Neither xclip nor wl-copy are available.".to_string())
}

fn generate_svg_string(
    code: &qrcode::QrCode,
    module_shape: i32,
    eye_frame_shape: i32,
    eye_ball_shape: i32,
    fg_style: i32,
    fg_color1: &str,
    fg_color2: &str,
    bg_style: i32,
    bg_color: &str,
    margin_size: i32,
) -> String {
    let qr_width = code.width();
    let margin = margin_size;
    let grid_size = qr_width + 2 * margin as usize;

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {0} {0}" width="512" height="512">"#,
        grid_size
    ));

    svg.push_str("<defs>");
    match fg_style {
        1 => { // Linear H
            svg.push_str(&format!(
                r#"<linearGradient id="fg-grad" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" /><stop offset="100%" stop-color="{}" /></linearGradient>"#,
                fg_color1, fg_color2
            ));
        }
        2 => { // Linear V
            svg.push_str(&format!(
                r#"<linearGradient id="fg-grad" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" stop-color="{}" /><stop offset="100%" stop-color="{}" /></linearGradient>"#,
                fg_color1, fg_color2
            ));
        }
        3 => { // Radial
            svg.push_str(&format!(
                r#"<radialGradient id="fg-grad" cx="50%" cy="50%" r="70%"><stop offset="0%" stop-color="{}" /><stop offset="100%" stop-color="{}" /></radialGradient>"#,
                fg_color1, fg_color2
            ));
        }
        _ => {}
    }
    svg.push_str("</defs>");

    let fill_color = if fg_style > 0 { "url(#fg-grad)" } else { fg_color1 };

    // Draw background
    if bg_style == 0 {
        svg.push_str(&format!(
            r#"<rect width="{0}" height="{0}" fill="{1}" />"#,
            grid_size, bg_color
        ));
    }

    svg.push_str(&format!(r#"<g transform="translate({}, {})">"#, margin, margin));

    let is_eye = |x: i32, y: i32| -> bool {
        let n = qr_width as i32;
        (x < 7 && y < 7) || (x >= n - 7 && y < 7) || (x < 7 && y >= n - 7)
    };

    // Draw standard modules
    for y in 0..qr_width {
        for x in 0..qr_width {
            if is_eye(x as i32, y as i32) {
                continue;
            }
            if code[(x, y)] == qrcode::Color::Dark {
                match module_shape {
                    0 => { // Square
                        svg.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="1" height="1" fill="{}" />"#,
                            x, y, fill_color
                        ));
                    }
                    1 => { // Rounded
                        svg.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="1" height="1" rx="0.28" ry="0.28" fill="{}" />"#,
                            x, y, fill_color
                        ));
                    }
                    _ => { // Circle
                        svg.push_str(&format!(
                            r#"<circle cx="{}.5" cy="{}.5" r="0.4" fill="{}" />"#,
                            x, y, fill_color
                        ));
                    }
                }
            }
        }
    }

    let draw_eye = |svg_str: &mut String, rx: i32, ry: i32| {
        match eye_frame_shape {
            0 => { // Square Frame
                svg_str.push_str(&format!(
                    r#"<path d="M {0},{1} h 7 v 7 h -7 z M {2},{3} h 5 v 5 h -5 z" fill-rule="evenodd" fill="{4}" />"#,
                    rx, ry, rx + 1, ry + 1, fill_color
                ));
            }
            1 => { // Rounded Frame
                svg_str.push_str(&format!(
                    r#"<rect x="{0}" y="{1}" width="7" height="7" rx="1.5" ry="1.5" fill="{2}" />"#,
                    rx, ry, fill_color
                ));
                let mask_color = if bg_style == 0 { bg_color } else { "white" };
                svg_str.push_str(&format!(
                    r#"<rect x="{0}" y="{1}" width="5" height="5" rx="0.5" ry="0.5" fill="{2}" />"#,
                    rx + 1, ry + 1, mask_color
                ));
            }
            _ => { // Circle Frame
                svg_str.push_str(&format!(
                    r#"<circle cx="{0}" cy="{1}" r="3.5" fill="{2}" />"#,
                    rx as f32 + 3.5, ry as f32 + 3.5, fill_color
                ));
                let mask_color = if bg_style == 0 { bg_color } else { "white" };
                svg_str.push_str(&format!(
                    r#"<circle cx="{0}" cy="{1}" r="2.5" fill="{2}" />"#,
                    rx as f32 + 3.5, ry as f32 + 3.5, mask_color
                ));
            }
        }

        match eye_ball_shape {
            0 => { // Square Ball
                svg_str.push_str(&format!(
                    r#"<rect x="{0}" y="{1}" width="3" height="3" fill="{2}" />"#,
                    rx + 2, ry + 2, fill_color
                ));
            }
            1 => { // Rounded Ball
                svg_str.push_str(&format!(
                    r#"<rect x="{0}" y="{1}" width="3" height="3" rx="0.8" ry="0.8" fill="{2}" />"#,
                    rx + 2, ry + 2, fill_color
                ));
            }
            _ => { // Circle Ball
                svg_str.push_str(&format!(
                    r#"<circle cx="{0}" cy="{1}" r="1.5" fill="{2}" />"#,
                    rx as f32 + 3.5, ry as f32 + 3.5, fill_color
                ));
            }
        }
    };

    let n = qr_width as i32;
    draw_eye(&mut svg, 0, 0); // Top-Left
    draw_eye(&mut svg, n - 7, 0); // Top-Right
    draw_eye(&mut svg, 0, n - 7); // Bottom-Left

    svg.push_str("</g>");
    svg.push_str("</svg>");
    svg
}

fn generate_and_render(ui: &AppWindow) -> Option<image::RgbaImage> {
    // 1. Gather properties
    let qr_type = ui.get_qr_type();
    let qr_text = ui.get_qr_text();

    let wifi_ssid = ui.get_wifi_ssid();
    let wifi_password = ui.get_wifi_password();
    let wifi_security = ui.get_wifi_security();

    let contact_name = ui.get_contact_name();
    let contact_phone = ui.get_contact_phone();
    let contact_email = ui.get_contact_email();
    let contact_org = ui.get_contact_org();
    let contact_address = ui.get_contact_address();

    let email_to = ui.get_email_to();
    let email_subject = ui.get_email_subject();
    let email_body = ui.get_email_body();

    let sms_phone = ui.get_sms_phone();
    let sms_message = ui.get_sms_message();

    let fg_style = ui.get_fg_style();
    let fg_color1 = ui.get_fg_color1();
    let fg_color2 = ui.get_fg_color2();
    let bg_style = ui.get_bg_style();
    let bg_color = ui.get_bg_color();

    let module_shape = ui.get_module_shape();
    let eye_frame_shape = ui.get_eye_frame_shape();
    let eye_ball_shape = ui.get_eye_ball_shape();

    let error_correction = ui.get_error_correction();
    let margin_size = ui.get_margin();

    let has_logo = ui.get_has_logo();
    let logo_path = ui.get_logo_path();
    let logo_size = ui.get_logo_size();
    let clear_logo_bg = ui.get_clear_logo_bg();

    // 2. Format content based on type
    let text_to_encode = match qr_type {
        0 => qr_text.to_string(),
        1 => {
            let sec = match wifi_security {
                0 => "WPA",
                1 => "WEP",
                _ => "nopass",
            };
            if sec == "nopass" {
                format!("WIFI:S:{};T:nopass;;", wifi_ssid)
            } else {
                format!("WIFI:S:{};T:{};P:{};;", wifi_ssid, sec, wifi_password)
            }
        }
        2 => {
            format!(
                "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL;TYPE=CELL:{}\nEMAIL;TYPE=PREF,INTERNET:{}\nORG:{}\nADR;TYPE=WORK:;;{};;;;\nEND:VCARD",
                contact_name, contact_phone, contact_email, contact_org, contact_address
            )
        }
        3 => {
            format!(
                "mailto:{}?subject={}&body={}",
                email_to,
                urlencoding::encode(&email_subject),
                urlencoding::encode(&email_body)
            )
        }
        4 => {
            format!("SMSTO:{}:{}", sms_phone, sms_message)
        }
        _ => qr_text.to_string(),
    };

    let ec_level = match error_correction {
        0 => EcLevel::L,
        1 => EcLevel::M,
        2 => EcLevel::Q,
        _ => EcLevel::H,
    };

    // 3. Generate QR modules
    let code = match qrcode::QrCode::with_error_correction_level(text_to_encode.as_bytes(), ec_level) {
        Ok(c) => c,
        Err(e) => {
            ui.set_status_message(format!("خطأ في ترميز البيانات: {}", e).into());
            ui.set_has_qr(false);
            return None;
        }
    };

    let qr_width = code.width();
    let margin = margin_size;
    let grid_size = qr_width + 2 * margin as usize;

    let img_size = 512;
    let cell_size = img_size as f32 / grid_size as f32;

    let mut img = image::RgbaImage::new(img_size, img_size);

    let fg1 = parse_hex_color(&fg_color1).unwrap_or(image::Rgba([99, 102, 241, 255]));
    let fg2 = parse_hex_color(&fg_color2).unwrap_or(image::Rgba([217, 70, 239, 255]));
    let bg = if bg_style == 1 {
        image::Rgba([0, 0, 0, 0])
    } else {
        parse_hex_color(&bg_color).unwrap_or(image::Rgba([255, 255, 255, 255]))
    };

    // Render pixel grid
    for py in 0..img_size {
        for px in 0..img_size {
            // Draw logo badge backdrop if applicable
            let logo_badge_cleared = if has_logo && clear_logo_bg {
                let badge_w = img_size as f32 * (logo_size as f32 + 4.0) / 100.0;
                let badge_h = img_size as f32 * (logo_size as f32 + 4.0) / 100.0;
                let bx1 = (img_size as f32 - badge_w) / 2.0;
                let bx2 = (img_size as f32 + badge_w) / 2.0;
                let by1 = (img_size as f32 - badge_h) / 2.0;
                let by2 = (img_size as f32 + badge_h) / 2.0;

                if px as f32 >= bx1 && px as f32 <= bx2 && py as f32 >= by1 && py as f32 <= by2 {
                    let rx = px as f32 - bx1;
                    let ry = py as f32 - by1;
                    let badge_r = badge_w * 0.15;
                    in_rounded_rect(rx, ry, badge_w, badge_h, badge_r)
                } else {
                    false
                }
            } else {
                false
            };

            if logo_badge_cleared {
                let badge_col = if bg_style == 1 { image::Rgba([255, 255, 255, 255]) } else { bg };
                img.put_pixel(px, py, badge_col);
                continue;
            }

            let fx = px as f32 / cell_size - margin as f32;
            let fy = py as f32 / cell_size - margin as f32;
            let gx = fx.floor() as i32;
            let gy = fy.floor() as i32;

            if gx < 0 || gx >= qr_width as i32 || gy < 0 || gy >= qr_width as i32 {
                img.put_pixel(px, py, bg);
                continue;
            }

            let cx = fx - gx as f32;
            let cy = fy - gy as f32;

            let in_top_left = gx < 7 && gy < 7;
            let in_top_right = gx >= qr_width as i32 - 7 && gx < qr_width as i32 && gy < 7;
            let in_bottom_left = gx < 7 && gy >= qr_width as i32 - 7 && gy < qr_width as i32;

            if in_top_left || in_top_right || in_bottom_left {
                let rx = if in_top_left { gx } else if in_top_right { gx - (qr_width as i32 - 7) } else { gx };
                let ry = if in_top_left { gy } else if in_top_right { gy } else { gy - (qr_width as i32 - 7) };

                let ex = cx + rx as f32;
                let ey = cy + ry as f32;

                let is_frame = rx == 0 || rx == 6 || ry == 0 || ry == 6;
                let is_ball = rx >= 2 && rx <= 4 && ry >= 2 && ry <= 4;

                if is_frame {
                    let inside_frame = match eye_frame_shape {
                        0 => true,
                        1 => {
                            let r = 1.5;
                            let outer = in_rounded_rect(ex, ey, 7.0, 7.0, r);
                            let inner = ex > 1.0 && ex < 6.0 && ey > 1.0 && ey < 6.0;
                            outer && !inner
                        }
                        _ => {
                            let d2 = (ex - 3.5).powi(2) + (ey - 3.5).powi(2);
                            d2 <= 3.5f32.powi(2) && d2 >= 2.2f32.powi(2)
                        }
                    };

                    if inside_frame {
                        let col = interpolate_color(fg1, fg2, px as f32, py as f32, img_size as f32, fg_style);
                        img.put_pixel(px, py, col);
                    } else {
                        img.put_pixel(px, py, bg);
                    }
                } else if is_ball {
                    let inside_ball = match eye_ball_shape {
                        0 => true,
                        1 => {
                            let bx = ex - 2.0;
                            let by = ey - 2.0;
                            in_rounded_rect(bx, by, 3.0, 3.0, 0.8)
                        }
                        _ => {
                            let d2 = (ex - 3.5).powi(2) + (ey - 3.5).powi(2);
                            d2 <= 1.6f32.powi(2)
                        }
                    };

                    if inside_ball {
                        let col = interpolate_color(fg1, fg2, px as f32, py as f32, img_size as f32, fg_style);
                        img.put_pixel(px, py, col);
                    } else {
                        img.put_pixel(px, py, bg);
                    }
                } else {
                    img.put_pixel(px, py, bg);
                }
            } else {
                let is_dark = code[(gx as usize, gy as usize)] == qrcode::Color::Dark;
                if is_dark {
                    let inside_module = match module_shape {
                        0 => true,
                        1 => in_rounded_rect(cx, cy, 1.0, 1.0, 0.28),
                        _ => {
                            let d2 = (cx - 0.5).powi(2) + (cy - 0.5).powi(2);
                            d2 <= 0.4f32.powi(2)
                        }
                    };

                    if inside_module {
                        let col = interpolate_color(fg1, fg2, px as f32, py as f32, img_size as f32, fg_style);
                        img.put_pixel(px, py, col);
                    } else {
                        img.put_pixel(px, py, bg);
                    }
                } else {
                    img.put_pixel(px, py, bg);
                }
            }
        }
    }

    // 4. Overlay logo image if enabled
    if has_logo && !logo_path.is_empty() {
        if let Ok(logo_img) = image::open(&logo_path.to_string()) {
            let logo_size_px = (img_size as f32 * logo_size as f32 / 100.0) as u32;
            let resized_logo = logo_img.resize(logo_size_px, logo_size_px, image::imageops::FilterType::Lanczos3);
            let logo_rgba = resized_logo.to_rgba8();

            let start_x = (img_size - logo_size_px) / 2;
            let start_y = (img_size - logo_size_px) / 2;

            for ly in 0..logo_size_px {
                for lx in 0..logo_size_px {
                    let px = start_x + lx;
                    let py = start_y + ly;
                    if px < img_size && py < img_size {
                        let logo_pixel = logo_rgba.get_pixel(lx, ly);
                        if logo_pixel[3] > 0 {
                            let alpha = logo_pixel[3] as f32 / 255.0;
                            let qr_pixel = img.get_pixel(px, py);
                            let blended = image::Rgba([
                                (qr_pixel[0] as f32 * (1.0 - alpha) + logo_pixel[0] as f32 * alpha) as u8,
                                (qr_pixel[1] as f32 * (1.0 - alpha) + logo_pixel[1] as f32 * alpha) as u8,
                                (qr_pixel[2] as f32 * (1.0 - alpha) + logo_pixel[2] as f32 * alpha) as u8,
                                (qr_pixel[3] as f32 * (1.0 - alpha) + logo_pixel[3] as f32 * alpha) as u8,
                            ]);
                            img.put_pixel(px, py, blended);
                        }
                    }
                }
            }
        }
    }

    // 5. Update UI properties
    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        img.as_raw(),
        img.width(),
        img.height(),
    );
    ui.set_qr_image(slint::Image::from_rgba8(buffer));
    ui.set_status_message("تم تحديث كود الـ QR ديناميكياً!".into());
    ui.set_has_qr(true);

    Some(img)
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let app_weak = app.as_weak();
    app.on_trigger_generate(move || {
        if let Some(app) = app_weak.upgrade() {
            generate_and_render(&app);
        }
    });

    let app_weak = app.as_weak();
    app.on_select_logo_file(move || {
        if let Some(app) = app_weak.upgrade() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Image Files", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
                app.set_logo_path(path.to_string_lossy().to_string().into());
                generate_and_render(&app);
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_save_png_file(move || {
        if let Some(app) = app_weak.upgrade() {
            if let Some(img) = generate_and_render(&app) {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .set_file_name("girsu_qrcode.png")
                    .save_file()
                {
                    match img.save(&path) {
                        Ok(_) => app.set_status_message(format!("تم حفظ ملف PNG في {}", path.display()).into()),
                        Err(e) => app.set_status_message(format!("فشل حفظ ملف PNG: {}", e).into()),
                    }
                }
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_save_svg_file(move || {
        if let Some(app) = app_weak.upgrade() {
            // Re-generate to get QrCode
            let qr_type = app.get_qr_type();
            let qr_text = app.get_qr_text();

            let wifi_ssid = app.get_wifi_ssid();
            let wifi_password = app.get_wifi_password();
            let wifi_security = app.get_wifi_security();

            let contact_name = app.get_contact_name();
            let contact_phone = app.get_contact_phone();
            let contact_email = app.get_contact_email();
            let contact_org = app.get_contact_org();
            let contact_address = app.get_contact_address();

            let email_to = app.get_email_to();
            let email_subject = app.get_email_subject();
            let email_body = app.get_email_body();

            let sms_phone = app.get_sms_phone();
            let sms_message = app.get_sms_message();

            let text_to_encode = match qr_type {
                0 => qr_text.to_string(),
                1 => {
                    let sec = match wifi_security {
                        0 => "WPA",
                        1 => "WEP",
                        _ => "nopass",
                    };
                    if sec == "nopass" {
                        format!("WIFI:S:{};T:nopass;;", wifi_ssid)
                    } else {
                        format!("WIFI:S:{};T:{};P:{};;", wifi_ssid, sec, wifi_password)
                    }
                }
                2 => {
                    format!(
                        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL;TYPE=CELL:{}\nEMAIL;TYPE=PREF,INTERNET:{}\nORG:{}\nADR;TYPE=WORK:;;{};;;;\nEND:VCARD",
                        contact_name, contact_phone, contact_email, contact_org, contact_address
                    )
                }
                3 => {
                    format!(
                        "mailto:{}?subject={}&body={}",
                        email_to,
                        urlencoding::encode(&email_subject),
                        urlencoding::encode(&email_body)
                    )
                }
                4 => {
                    format!("SMSTO:{}:{}", sms_phone, sms_message)
                }
                _ => qr_text.to_string(),
            };

            let ec_level = match app.get_error_correction() {
                0 => EcLevel::L,
                1 => EcLevel::M,
                2 => EcLevel::Q,
                _ => EcLevel::H,
            };

            if let Ok(code) = qrcode::QrCode::with_error_correction_level(text_to_encode.as_bytes(), ec_level) {
                let svg_content = generate_svg_string(
                    &code,
                    app.get_module_shape(),
                    app.get_eye_frame_shape(),
                    app.get_eye_ball_shape(),
                    app.get_fg_style(),
                    &app.get_fg_color1(),
                    &app.get_fg_color2(),
                    app.get_bg_style(),
                    &app.get_bg_color(),
                    app.get_margin(),
                );

                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("SVG Graphic", &["svg"])
                    .set_file_name("girsu_qrcode.svg")
                    .save_file()
                {
                    match File::create(&path) {
                        Ok(mut file) => {
                            let _ = file.write_all(svg_content.as_bytes());
                            app.set_status_message(format!("تم حفظ ملف SVG في {}", path.display()).into());
                        }
                        Err(e) => app.set_status_message(format!("فشل حفظ ملف SVG: {}", e).into()),
                    }
                }
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_copy_to_clipboard(move || {
        if let Some(app) = app_weak.upgrade() {
            // Copy both structured plain text AND the image (if Linux tools exist)
            let qr_type = app.get_qr_type();
            let qr_text = app.get_qr_text();

            let wifi_ssid = app.get_wifi_ssid();
            let wifi_password = app.get_wifi_password();
            let wifi_security = app.get_wifi_security();

            let contact_name = app.get_contact_name();
            let contact_phone = app.get_contact_phone();
            let contact_email = app.get_contact_email();
            let contact_org = app.get_contact_org();
            let contact_address = app.get_contact_address();

            let email_to = app.get_email_to();
            let email_subject = app.get_email_subject();
            let email_body = app.get_email_body();

            let sms_phone = app.get_sms_phone();
            let sms_message = app.get_sms_message();

            let text_to_encode = match qr_type {
                0 => qr_text.to_string(),
                1 => {
                    let sec = match wifi_security {
                        0 => "WPA",
                        1 => "WEP",
                        _ => "nopass",
                    };
                    if sec == "nopass" {
                        format!("WIFI:S:{};T:nopass;;", wifi_ssid)
                    } else {
                        format!("WIFI:S:{};T:{};P:{};;", wifi_ssid, sec, wifi_password)
                    }
                }
                2 => {
                    format!(
                        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL;TYPE=CELL:{}\nEMAIL;TYPE=PREF,INTERNET:{}\nORG:{}\nADR;TYPE=WORK:;;{};;;;\nEND:VCARD",
                        contact_name, contact_phone, contact_email, contact_org, contact_address
                    )
                }
                3 => {
                    format!(
                        "mailto:{}?subject={}&body={}",
                        email_to,
                        urlencoding::encode(&email_subject),
                        urlencoding::encode(&email_body)
                    )
                }
                4 => {
                    format!("SMSTO:{}:{}", sms_phone, sms_message)
                }
                _ => qr_text.to_string(),
            };

            // Copy Text first
            let text_copy_result = copy_text_to_clipboard_linux(&text_to_encode);

            // Copy Image next by saving to a temp file and copying it
            if let Some(img) = generate_and_render(&app) {
                let temp_dir = std::env::temp_dir();
                let temp_path = temp_dir.join("girsu_qrcode_clipboard.png");
                if img.save(&temp_path).is_ok() {
                    let path_str = temp_path.to_string_lossy().to_string();
                    match copy_png_to_clipboard_linux(&path_str) {
                        Ok(_) => app.set_status_message("تم نسخ صورة كود الـ QR إلى الحافظة بنجاح!".into()),
                        Err(e) => {
                            if text_copy_result.is_ok() {
                                app.set_status_message("تم نسخ النص الخام للكود! (نسخ الصورة يتطلب تثبيت xclip أو wl-clipboard)".into());
                            } else {
                                app.set_status_message(format!("فشل النسخ للحافظة: {}", e).into());
                            }
                        }
                    }
                }
            }
        }
    });

    // Run first-time generation on startup to show a beautiful default QR code
    generate_and_render(&app);

    app.run()
}
