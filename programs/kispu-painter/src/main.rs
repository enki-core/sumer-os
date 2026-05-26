slint::include_modules!();

use slint::{SharedPixelBuffer, Rgba8Pixel, Image};
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

#[derive(Clone)]
struct Layer {
    pixels: Vec<Rgba8Pixel>,
    opacity: f32,
    visible: bool,
}

impl Layer {
    fn new(width: usize, height: usize, transparent: bool) -> Self {
        let color = if transparent {
            Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 }
        } else {
            Rgba8Pixel { r: 255, g: 255, b: 255, a: 255 }
        };
        Self {
            pixels: vec![color; width * height],
            opacity: 1.0,
            visible: true,
        }
    }
}

#[derive(Clone)]
struct LayerState {
    pixels: Vec<Rgba8Pixel>,
    opacity: f32,
    visible: bool,
}

#[derive(Clone)]
struct UndoState {
    width: usize,
    height: usize,
    layers: Vec<LayerState>,
}

struct AppState {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
    active_layer: usize,
    current_color: Rgba8Pixel,
    tool: String,
    brush_size: usize,
    symmetry_mode: String,
    fill_shapes: bool,

    // Undo/Redo stacks
    undo_stack: Vec<UndoState>,
    redo_stack: Vec<UndoState>,

    // Drawing state
    is_drawing: bool,
    last_pos: Option<(i32, i32)>,
    shape_start: Option<(i32, i32)>,
    pre_shape_layers: Option<Vec<Layer>>,
}

impl AppState {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: vec![
                Layer::new(width, height, false), // Background Layer (White)
                Layer::new(width, height, true),  // Draw Layer (Transparent)
            ],
            active_layer: 1, // Draw Layer active by default
            current_color: Rgba8Pixel { r: 0, g: 0, b: 0, a: 255 },
            tool: "pencil".to_string(),
            brush_size: 1,
            symmetry_mode: "none".to_string(),
            fill_shapes: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            is_drawing: false,
            last_pos: None,
            shape_start: None,
            pre_shape_layers: None,
        }
    }

    fn save_undo_state(&mut self) {
        let state = UndoState {
            width: self.width,
            height: self.height,
            layers: self.layers.iter().map(|l| LayerState {
                pixels: l.pixels.clone(),
                opacity: l.opacity,
                visible: l.visible,
            }).collect(),
        };
        self.undo_stack.push(state);
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            let current = UndoState {
                width: self.width,
                height: self.height,
                layers: self.layers.iter().map(|l| LayerState {
                    pixels: l.pixels.clone(),
                    opacity: l.opacity,
                    visible: l.visible,
                }).collect(),
            };
            self.redo_stack.push(current);

            self.width = prev.width;
            self.height = prev.height;
            self.layers = prev.layers.iter().map(|l| Layer {
                pixels: l.pixels.clone(),
                opacity: l.opacity,
                visible: l.visible,
            }).collect();
            true
        } else {
            false
        }
    }

    fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            let current = UndoState {
                width: self.width,
                height: self.height,
                layers: self.layers.iter().map(|l| LayerState {
                    pixels: l.pixels.clone(),
                    opacity: l.opacity,
                    visible: l.visible,
                }).collect(),
            };
            self.undo_stack.push(current);

            self.width = next.width;
            self.height = next.height;
            self.layers = next.layers.iter().map(|l| Layer {
                pixels: l.pixels.clone(),
                opacity: l.opacity,
                visible: l.visible,
            }).collect();
            true
        } else {
            false
        }
    }

    fn resize_canvas(&mut self, new_w: usize, new_h: usize) {
        self.save_undo_state();

        for (idx, layer) in self.layers.iter_mut().enumerate() {
            let fill_color = if idx == 0 {
                Rgba8Pixel { r: 255, g: 255, b: 255, a: 255 } // background white
            } else {
                Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 } // drawing transparent
            };
            let mut new_pixels = vec![fill_color; new_w * new_h];

            // Copy old pixels centered
            let offset_x = (new_w as i32 - self.width as i32) / 2;
            let offset_y = (new_h as i32 - self.height as i32) / 2;

            for y in 0..self.height {
                for x in 0..self.width {
                    let old_idx = y * self.width + x;
                    let nx = x as i32 + offset_x;
                    let ny = y as i32 + offset_y;

                    if nx >= 0 && nx < new_w as i32 && ny >= 0 && ny < new_h as i32 {
                        let new_idx = ny as usize * new_w + nx as usize;
                        new_pixels[new_idx] = layer.pixels[old_idx];
                    }
                }
            }
            layer.pixels = new_pixels;
        }

        self.width = new_w;
        self.height = new_h;
    }

    fn clear_canvas(&mut self) {
        self.save_undo_state();
        for (idx, layer) in self.layers.iter_mut().enumerate() {
            let color = if idx == 0 {
                Rgba8Pixel { r: 255, g: 255, b: 255, a: 255 }
            } else {
                Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 }
            };
            layer.pixels.fill(color);
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Rgba8Pixel {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y * self.width as i32 + x) as usize;
            self.layers[self.active_layer].pixels[idx]
        } else {
            Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 }
        }
    }

    fn set_pixel_raw(&mut self, x: i32, y: i32, color: Rgba8Pixel) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y * self.width as i32 + x) as usize;
            self.layers[self.active_layer].pixels[idx] = color;
        }
    }

    fn set_pixel_with_symmetry(&mut self, x: i32, y: i32, color: Rgba8Pixel) {
        self.set_pixel_raw(x, y, color);

        let w = self.width as i32;
        let h = self.height as i32;

        match self.symmetry_mode.as_str() {
            "vertical" => {
                self.set_pixel_raw(w - 1 - x, y, color);
            }
            "horizontal" => {
                self.set_pixel_raw(x, h - 1 - y, color);
            }
            "both" => {
                self.set_pixel_raw(w - 1 - x, y, color);
                self.set_pixel_raw(x, h - 1 - y, color);
                self.set_pixel_raw(w - 1 - x, h - 1 - y, color);
            }
            _ => {}
        }
    }

    fn paint_brush_footprint(&mut self, xc: i32, yc: i32, color: Rgba8Pixel) {
        let size = self.brush_size as i32;
        if size <= 1 {
            self.set_pixel_with_symmetry(xc, yc, color);
        } else {
            let half = size / 2;
            let start_x = xc - half;
            let start_y = yc - half;

            for dy in 0..size {
                for dx in 0..size {
                    // Check circular brush profile if size > 2
                    if size > 2 {
                        let rx = dx - half;
                        let ry = dy - half;
                        if rx*rx + ry*ry > half*half {
                            continue; // skip corners for circular brush
                        }
                    }
                    self.set_pixel_with_symmetry(start_x + dx, start_y + dy, color);
                }
            }
        }
    }

    fn draw_line_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba8Pixel) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            self.paint_brush_footprint(x, y, color);
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x1 { break; }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y1 { break; }
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_rect_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba8Pixel, fill: bool) {
        let min_x = x0.min(x1);
        let max_x = x0.max(x1);
        let min_y = y0.min(y1);
        let max_y = y0.max(y1);

        if fill {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    self.paint_brush_footprint(x, y, color);
                }
            }
        } else {
            // Draw top & bottom edges
            for x in min_x..=max_x {
                self.paint_brush_footprint(x, min_y, color);
                self.paint_brush_footprint(x, max_y, color);
            }
            // Draw left & right edges
            for y in min_y..=max_y {
                self.paint_brush_footprint(min_x, y, color);
                self.paint_brush_footprint(max_x, y, color);
            }
        }
    }

    fn draw_circle_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba8Pixel, fill: bool) {
        let dx = (x1 - x0) as f32;
        let dy = (y1 - y0) as f32;
        let radius = (dx * dx + dy * dy).sqrt() as i32;

        if radius <= 0 {
            self.paint_brush_footprint(x0, y0, color);
            return;
        }

        if fill {
            for y in -radius..=radius {
                for x in -radius..=radius {
                    if x*x + y*y <= radius*radius {
                        self.paint_brush_footprint(x0 + x, y0 + y, color);
                    }
                }
            }
        } else {
            let mut x = 0;
            let mut y = radius;
            let mut d = 3 - 2 * radius;
            self.draw_circle_octants(x0, y0, x, y, color);
            while y >= x {
                x += 1;
                if d > 0 {
                    y -= 1;
                    d = d + 4 * (x - y) + 10;
                } else {
                    d = d + 4 * x + 6;
                }
                self.draw_circle_octants(x0, y0, x, y, color);
            }
        }
    }

    fn draw_circle_octants(&mut self, xc: i32, yc: i32, x: i32, y: i32, color: Rgba8Pixel) {
        self.paint_brush_footprint(xc + x, yc + y, color);
        self.paint_brush_footprint(xc - x, yc + y, color);
        self.paint_brush_footprint(xc + x, yc - y, color);
        self.paint_brush_footprint(xc - x, yc - y, color);
        self.paint_brush_footprint(xc + y, yc + x, color);
        self.paint_brush_footprint(xc - y, yc + x, color);
        self.paint_brush_footprint(xc + y, yc - x, color);
        self.paint_brush_footprint(xc - y, yc - x, color);
    }

    fn flood_fill(&mut self, start_x: i32, start_y: i32, target_color: Rgba8Pixel, replacement_color: Rgba8Pixel) {
        if target_color == replacement_color {
            return;
        }

        let w = self.width as i32;
        let h = self.height as i32;

        if start_x < 0 || start_x >= w || start_y < 0 || start_y >= h {
            return;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_x, start_y));

        let active_idx = self.active_layer;

        let get_px = |layers: &[Layer], x: i32, y: i32| -> Rgba8Pixel {
            let idx = (y * w + x) as usize;
            layers[active_idx].pixels[idx]
        };

        let start_color = get_px(&self.layers, start_x, start_y);
        if start_color != target_color {
            return;
        }

        while let Some((cx, cy)) = queue.pop_front() {
            if get_px(&self.layers, cx, cy) != target_color {
                continue;
            }

            self.set_pixel_with_symmetry(cx, cy, replacement_color);

            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = cx + dx;
                let ny = cy + dy;
                if nx >= 0 && nx < w && ny >= 0 && ny < h {
                    if get_px(&self.layers, nx, ny) == target_color {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    fn merge_display(&self) -> SharedPixelBuffer<Rgba8Pixel> {
        let mut display_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(self.width as u32, self.height as u32);
        let display_slice = display_buffer.make_mut_slice();

        // 1. Draw Checkerboard background representing transparency
        for y in 0..self.height {
            for x in 0..self.width {
                let is_dark_square = ((x / 4) + (y / 4)) % 2 == 0;
                let color = if is_dark_square {
                    Rgba8Pixel { r: 240, g: 240, b: 240, a: 255 }
                } else {
                    Rgba8Pixel { r: 255, g: 255, b: 255, a: 255 }
                };
                display_slice[y * self.width + x] = color;
            }
        }

        // 2. Blend visible layers onto the display buffer
        for layer in &self.layers {
            if !layer.visible || layer.opacity <= 0.0 {
                continue;
            }
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = y * self.width + x;
                    let src = layer.pixels[idx];
                    let src_alpha = (src.a as f32 / 255.0) * layer.opacity;

                    if src_alpha <= 0.0 {
                        continue;
                    }

                    let dest = &mut display_slice[idx];

                    if src_alpha >= 1.0 {
                        *dest = Rgba8Pixel { r: src.r, g: src.g, b: src.b, a: 255 };
                    } else {
                        // Standard Alpha Blending formula
                        let dest_alpha = dest.a as f32 / 255.0;
                        let out_alpha = src_alpha + dest_alpha * (1.0 - src_alpha);
                        
                        let r = (src.r as f32 * src_alpha + dest.r as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let g = (src.g as f32 * src_alpha + dest.g as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let b = (src.b as f32 * src_alpha + dest.b as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;

                        *dest = Rgba8Pixel {
                            r: r.clamp(0.0, 255.0) as u8,
                            g: g.clamp(0.0, 255.0) as u8,
                            b: b.clamp(0.0, 255.0) as u8,
                            a: (out_alpha * 255.0).clamp(0.0, 255.0) as u8,
                        };
                    }
                }
            }
        }

        display_buffer
    }

    fn merge_raw_export(&self) -> Vec<Rgba8Pixel> {
        let mut export_pixels = vec![Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 }; self.width * self.height];

        for layer in &self.layers {
            if !layer.visible || layer.opacity <= 0.0 {
                continue;
            }
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = y * self.width + x;
                    let src = layer.pixels[idx];
                    let src_alpha = (src.a as f32 / 255.0) * layer.opacity;

                    if src_alpha <= 0.0 {
                        continue;
                    }

                    let dest = &mut export_pixels[idx];

                    if src_alpha >= 1.0 {
                        *dest = Rgba8Pixel { r: src.r, g: src.g, b: src.b, a: (src_alpha * 255.0) as u8 };
                    } else {
                        let dest_alpha = dest.a as f32 / 255.0;
                        let out_alpha = src_alpha + dest_alpha * (1.0 - src_alpha);
                        
                        if out_alpha > 0.0 {
                            let r = (src.r as f32 * src_alpha + dest.r as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                            let g = (src.g as f32 * src_alpha + dest.g as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                            let b = (src.b as f32 * src_alpha + dest.b as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;

                            *dest = Rgba8Pixel {
                                r: r.clamp(0.0, 255.0) as u8,
                                g: g.clamp(0.0, 255.0) as u8,
                                b: b.clamp(0.0, 255.0) as u8,
                                a: (out_alpha * 255.0).clamp(0.0, 255.0) as u8,
                            };
                        }
                    }
                }
            }
        }
        export_pixels
    }
}

fn select_save_path() -> Option<String> {
    let output = std::process::Command::new("zenity")
        .args(&[
            "--file-selection",
            "--save",
            "--confirm-overwrite",
            "--title=حفظ لوحة الرسم | Save Painting",
            "--file-filter=PNG Images | *.png",
        ])
        .output();
        
    if let Ok(out) = output {
        if out.status.success() {
            let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path_str.is_empty() {
                if !path_str.to_lowercase().ends_with(".png") {
                    return Some(format!("{}.png", path_str));
                }
                return Some(path_str);
            }
        }
    }
    None
}

fn select_open_path() -> Option<String> {
    let output = std::process::Command::new("zenity")
        .args(&[
            "--file-selection",
            "--title=فتح صورة للرسم | Open Image",
            "--file-filter=Image Files (PNG, JPG, BMP) | *.png *.jpg *.jpeg *.bmp",
        ])
        .output();
        
    if let Ok(out) = output {
        if out.status.success() {
            let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(path_str);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();

    // Size mappings for SizeIndex resolutions:
    // Index 0: 16x16, 1: 32x32, 2: 64x64, 3: 128x128, 4: 256x256, 5: 512x512
    let size_mapping = [16, 32, 64, 128, 256, 512];

    let state = Rc::new(RefCell::new(AppState::new(32, 32)));

    // Initialize first display render
    {
        let buffer = state.borrow().merge_display();
        app.set_canvas_image(Image::from_rgba8(buffer));
    }

    // Connect Slint Callbacks
    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_pointer_event(move |event_kind, mouse_x, mouse_y, width, height| {
        let mut s = state_ctrl.borrow_mut();

        // Calculate clamped pixel coordinates
        let canvas_w = s.width as f32;
        let canvas_h = s.height as f32;
        
        let px = ((mouse_x / width) * canvas_w).floor() as i32;
        let py = ((mouse_y / height) * canvas_h).floor() as i32;
        
        let x = px.clamp(0, s.width as i32 - 1);
        let y = py.clamp(0, s.height as i32 - 1);

        if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_cursor_x(x);
            app.set_cursor_y(y);
            s.brush_size = app.get_brush_size() as usize;
            s.fill_shapes = app.get_fill_shapes();
        }

        let fill_shapes = s.fill_shapes;

        let color = if s.tool == "eraser" {
            Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 } // eraser paints transparency
        } else {
            s.current_color
        };

        match event_kind.as_str() {
            "down" => {
                s.save_undo_state();
                s.is_drawing = true;
                s.last_pos = Some((x, y));

                if s.tool == "line" || s.tool == "rect" || s.tool == "circle" {
                    s.shape_start = Some((x, y));
                    s.pre_shape_layers = Some(s.layers.clone());
                } else if s.tool == "fill" {
                    let target_color = s.get_pixel(x, y);
                    s.flood_fill(x, y, target_color, color);
                } else if s.tool == "dropper" {
                    let sampled = s.get_pixel(x, y);
                    s.current_color = sampled;
                    if let Some(app) = app_weak_ctrl.upgrade() {
                        app.set_color_r(sampled.r as f32 / 255.0);
                        app.set_color_g(sampled.g as f32 / 255.0);
                        app.set_color_b(sampled.b as f32 / 255.0);
                        app.set_color_a(sampled.a as f32 / 255.0);
                        app.set_status_text("تم سحب اللون!".into());
                    }
                } else {
                    s.paint_brush_footprint(x, y, color);
                }
            }
            "move" => {
                if s.is_drawing {
                    if s.tool == "line" || s.tool == "rect" || s.tool == "circle" {
                        if let (Some(start), Some(ref pre)) = (s.shape_start, &s.pre_shape_layers) {
                            s.layers = pre.clone(); // restore pristine state
                            
                            if s.tool == "line" {
                                s.draw_line_pixels(start.0, start.1, x, y, color);
                            } else if s.tool == "rect" {
                                s.draw_rect_pixels(start.0, start.1, x, y, color, fill_shapes);
                            } else if s.tool == "circle" {
                                s.draw_circle_pixels(start.0, start.1, x, y, color, fill_shapes);
                            }
                        }
                    } else if s.tool == "pencil" || s.tool == "eraser" {
                        if let Some(last) = s.last_pos {
                            s.draw_line_pixels(last.0, last.1, x, y, color);
                        } else {
                            s.paint_brush_footprint(x, y, color);
                        }
                        s.last_pos = Some((x, y));
                    }
                }
            }
            "up" => {
                if s.is_drawing {
                    if s.tool == "line" || s.tool == "rect" || s.tool == "circle" {
                        if let (Some(start), Some(ref pre)) = (s.shape_start, &s.pre_shape_layers) {
                            s.layers = pre.clone(); // restore pristine state
                            
                            if s.tool == "line" {
                                s.draw_line_pixels(start.0, start.1, x, y, color);
                            } else if s.tool == "rect" {
                                s.draw_rect_pixels(start.0, start.1, x, y, color, fill_shapes);
                            } else if s.tool == "circle" {
                                s.draw_circle_pixels(start.0, start.1, x, y, color, fill_shapes);
                            }
                        }
                    }
                    s.is_drawing = false;
                    s.last_pos = None;
                    s.shape_start = None;
                    s.pre_shape_layers = None;
                }
            }
            _ => {}
        }

        // Trigger visual redraw update
        let buffer = s.merge_display();
        if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_canvas_image(Image::from_rgba8(buffer));
        }
    });

    let state_ctrl = state.clone();
    app.on_change_tool(move |tool_name| {
        let mut s = state_ctrl.borrow_mut();
        s.tool = tool_name.to_string();
    });

    let state_ctrl = state.clone();
    app.on_change_symmetry(move |sym_mode| {
        let mut s = state_ctrl.borrow_mut();
        s.symmetry_mode = sym_mode.to_string();
    });

    let state_ctrl = state.clone();
    app.on_change_color(move |r, g, b, a| {
        let mut s = state_ctrl.borrow_mut();
        s.current_color = Rgba8Pixel { r: r as u8, g: g as u8, b: b as u8, a: a as u8 };
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_toggle_layer_visibility(move |layer_idx, visible| {
        let mut s = state_ctrl.borrow_mut();
        let layer_idx = layer_idx as usize;
        if layer_idx < s.layers.len() {
            s.layers[layer_idx].visible = visible;
            let buffer = s.merge_display();
            if let Some(app) = app_weak_ctrl.upgrade() {
                app.set_canvas_image(Image::from_rgba8(buffer));
            }
        }
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_change_layer_opacity(move |layer_idx, opacity| {
        let mut s = state_ctrl.borrow_mut();
        let layer_idx = layer_idx as usize;
        if layer_idx < s.layers.len() {
            s.layers[layer_idx].opacity = opacity;
            let buffer = s.merge_display();
            if let Some(app) = app_weak_ctrl.upgrade() {
                app.set_canvas_image(Image::from_rgba8(buffer));
            }
        }
    });

    let state_ctrl = state.clone();
    app.on_change_active_layer(move |layer_idx| {
        let mut s = state_ctrl.borrow_mut();
        let layer_idx = layer_idx as usize;
        if layer_idx < s.layers.len() {
            s.active_layer = layer_idx;
        }
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_clear_canvas(move || {
        let mut s = state_ctrl.borrow_mut();
        s.clear_canvas();
        let buffer = s.merge_display();
        if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_canvas_image(Image::from_rgba8(buffer));
            app.set_status_text("تم مسح لوحة الرسم".into());
        }
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_resize_canvas(move |size_index| {
        let mut s = state_ctrl.borrow_mut();
        let idx = size_index.clamp(0, 5) as usize;
        let new_size = size_mapping[idx];
        s.resize_canvas(new_size, new_size);

        if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_canvas_width(new_size as i32);
            app.set_canvas_height(new_size as i32);
            
            let buffer = s.merge_display();
            app.set_canvas_image(Image::from_rgba8(buffer));
            app.set_status_text(format!("تم تغيير المقاس إلى {}x{}", new_size, new_size).into());
        }
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_undo(move || {
        let mut s = state_ctrl.borrow_mut();
        let success = s.undo();
        if let Some(app) = app_weak_ctrl.upgrade() {
            if success {
                app.set_canvas_width(s.width as i32);
                app.set_canvas_height(s.height as i32);
                
                // update size buttons indices to match restored state
                let restored_size = s.width;
                for (i, &size) in size_mapping.iter().enumerate() {
                    if size == restored_size {
                        app.set_size_index(i as i32);
                        break;
                    }
                }

                let buffer = s.merge_display();
                app.set_canvas_image(Image::from_rgba8(buffer));
                app.set_status_text("تم التراجع عن العملية".into());
            } else {
                app.set_status_text("لا يوجد عمليات للتراجع عنها".into());
            }
        }
    });

    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_redo(move || {
        let mut s = state_ctrl.borrow_mut();
        let success = s.redo();
        if let Some(app) = app_weak_ctrl.upgrade() {
            if success {
                app.set_canvas_width(s.width as i32);
                app.set_canvas_height(s.height as i32);
                
                let restored_size = s.width;
                for (i, &size) in size_mapping.iter().enumerate() {
                    if size == restored_size {
                        app.set_size_index(i as i32);
                        break;
                    }
                }

                let buffer = s.merge_display();
                app.set_canvas_image(Image::from_rgba8(buffer));
                app.set_status_text("تم إعادة تكرار العملية".into());
            } else {
                app.set_status_text("لا يوجد عمليات لتكرارها".into());
            }
        }
    });

    // Save image callback (uses Zenity dialog & image crate)
    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_save_image(move || {
        let s = state_ctrl.borrow();
        if let Some(path) = select_save_path() {
            let export_pixels = s.merge_raw_export();
            let mut img_buf = image::ImageBuffer::new(s.width as u32, s.height as u32);
            for y in 0..s.height {
                for x in 0..s.width {
                    let pixel = export_pixels[y * s.width + x];
                    img_buf.put_pixel(x as u32, y as u32, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
                }
            }

            if let Err(e) = img_buf.save(&path) {
                if let Some(app) = app_weak_ctrl.upgrade() {
                    app.set_status_text(format!("خطأ أثناء حفظ الملف: {}", e).into());
                }
            } else if let Some(app) = app_weak_ctrl.upgrade() {
                app.set_status_text("تم حفظ اللوحة بنجاح!".into());
            }
        } else if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_status_text("تم إلغاء عملية الحفظ".into());
        }
    });

    // Open image callback
    let state_ctrl = state.clone();
    let app_weak_ctrl = app_weak.clone();
    app.on_open_image(move || {
        if let Some(path) = select_open_path() {
            match image::open(&path) {
                Ok(img) => {
                    let img_rgba = img.to_rgba8();
                    let (w, h) = img_rgba.dimensions();
                    
                    let mut s = state_ctrl.borrow_mut();
                    s.save_undo_state();
                    
                    s.width = w as usize;
                    s.height = h as usize;
                    
                    // Create new layers with loaded dimensions
                    s.layers = vec![
                        Layer::new(s.width, s.height, false), // Background
                        Layer {
                            pixels: img_rgba.pixels().map(|p| Rgba8Pixel { r: p[0], g: p[1], b: p[2], a: p[3] }).collect(),
                            opacity: 1.0,
                            visible: true,
                        }
                    ];
                    s.active_layer = 1;

                    if let Some(app) = app_weak_ctrl.upgrade() {
                        app.set_canvas_width(s.width as i32);
                        app.set_canvas_height(s.height as i32);
                        
                        // Try to select nearest size mapping index in buttons
                        let mut selected_idx = 1; // default to 32
                        for (i, &size) in size_mapping.iter().enumerate() {
                            if size >= s.width {
                                selected_idx = i;
                                break;
                            }
                        }
                        app.set_size_index(selected_idx as i32);

                        let buffer = s.merge_display();
                        app.set_canvas_image(Image::from_rgba8(buffer));
                        app.set_status_text("تم تحميل الصورة بنجاح".into());
                    }
                }
                Err(e) => {
                    if let Some(app) = app_weak_ctrl.upgrade() {
                        app.set_status_text(format!("خطأ أثناء فتح الملف: {}", e).into());
                    }
                }
            }
        } else if let Some(app) = app_weak_ctrl.upgrade() {
            app.set_status_text("تم إلغاء عملية الفتح".into());
        }
    });

    app.run()?;
    Ok(())
}
