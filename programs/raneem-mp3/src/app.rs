use slint::{ComponentHandle, Model};
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::AppWindow;
use crate::audio::{SimpleRng, read_mp3_meta, parse_lrc, play_audio_file, pause_audio, resume_audio, LyricLine};

#[derive(Clone)]
struct TrackData {
    file_path: String,
    title: String,
    artist: String,
    duration_secs: i32,
    lyrics: Vec<LyricLine>,
}

fn rc_to_slint_model(vec: Vec<f32>) -> slint::ModelRc<f32> {
    slint::ModelRc::new(slint::VecModel::from(vec))
}

fn format_duration(secs: i32) -> String {
    let mins = secs / 60;
    let remaining_secs = secs % 60;
    format!("{:02}:{:02}", mins, remaining_secs)
}

fn update_slint_tracks(app: &AppWindow, tracks: &[TrackData]) {
    let mut slint_tracks = Vec::new();
    let art_colors = vec![
        (slint::Color::from_rgb_u8(0x3b, 0x82, 0xf6), slint::Color::from_rgb_u8(0x1d, 0x4e, 0xd8)),
        (slint::Color::from_rgb_u8(0xec, 0x48, 0x99), slint::Color::from_rgb_u8(0xbe, 0x18, 0x5d)),
        (slint::Color::from_rgb_u8(0x10, 0xb9, 0x81), slint::Color::from_rgb_u8(0x04, 0x78, 0x57)),
        (slint::Color::from_rgb_u8(0xf5, 0x9e, 0x0b), slint::Color::from_rgb_u8(0xb4, 0x53, 0x09)),
        (slint::Color::from_rgb_u8(0x8b, 0x5c, 0xf6), slint::Color::from_rgb_u8(0x6d, 0x28, 0xd9)),
    ];
    for (idx, td) in tracks.iter().enumerate() {
        let colors = &art_colors[idx % art_colors.len()];
        slint_tracks.push(crate::Track {
            title: td.title.clone().into(),
            artist: td.artist.clone().into(),
            duration: format_duration(td.duration_secs).into(),
            duration_secs: td.duration_secs,
            album_art_color_start: colors.0.clone(),
            album_art_color_end: colors.1.clone(),
        });
    }
    let tracks_model = slint::ModelRc::new(slint::VecModel::from(slint_tracks));
    app.set_tracks(tracks_model);
}

fn scan_library() -> Vec<TrackData> {
    let mut tracks = Vec::new();
    
    // Directories to scan
    let dirs = vec![
        PathBuf::from("ui/assets"),
        PathBuf::from("programs/raneem-mp3/ui/assets"),
        PathBuf::from("/home/debian/Music"),
    ];
    
    for dir in dirs {
        if dir.exists() && dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "mp3") {
                        let meta = read_mp3_meta(&path);
                        
                        // Look for a matching .lrc file
                        let mut lrc_path = path.clone();
                        lrc_path.set_extension("lrc");
                        let lyrics = if lrc_path.exists() {
                            parse_lrc(&lrc_path)
                        } else {
                            Vec::new()
                        };
                        
                        tracks.push(TrackData {
                            file_path: path.to_string_lossy().to_string(),
                            title: meta.title,
                            artist: meta.artist,
                            duration_secs: meta.duration_secs,
                            lyrics,
                        });
                    }
                }
            }
        }
    }
    
    // If no tracks found, add Mesopotamian mocks
    if tracks.is_empty() {
        let mock_path = "ui/assets/audio.mp3";
        tracks.push(TrackData {
            file_path: mock_path.to_string(),
            title: "ألحان القيثارة السومرية".to_string(),
            artist: "أور القديمة".to_string(),
            duration_secs: 225,
            lyrics: vec![
                LyricLine { time_secs: 0, text: "🏺 ألحان القيثارة السومرية تعزف...".to_string() },
                LyricLine { time_secs: 10, text: "🎶 تتردد الأصداء في معابد أور...".to_string() },
                LyricLine { time_secs: 25, text: "✨ نغمة مقدسة تتحدى الزمن...".to_string() },
            ],
        });
        tracks.push(TrackData {
            file_path: mock_path.to_string(),
            title: "ملحمة جلجامش الموسيقية".to_string(),
            artist: "أنكيدو".to_string(),
            duration_secs: 312,
            lyrics: vec![
                LyricLine { time_secs: 0, text: "⚔️ - ملحمة جلجامش الخالدة...".to_string() },
                LyricLine { time_secs: 15, text: "🌿 بحثاً عن سر الخلود في البراري...".to_string() },
                LyricLine { time_secs: 35, text: "🏰 سور أور العظيم يشهد التاريخ...".to_string() },
            ],
        });
        tracks.push(TrackData {
            file_path: mock_path.to_string(),
            title: "نسيم حدائق بابل المعلقة".to_string(),
            artist: "عشتار".to_string(),
            duration_secs: 260,
            lyrics: vec![
                LyricLine { time_secs: 0, text: "🌸 نسيم حدائق بابل المعلقة...".to_string() },
                LyricLine { time_secs: 20, text: "🍃 المياه تتدفق من أعالي السلالم...".to_string() },
            ],
        });
        tracks.push(TrackData {
            file_path: mock_path.to_string(),
            title: "نهر دجلة الخالد".to_string(),
            artist: "حضارة الرافدين".to_string(),
            duration_secs: 365,
            lyrics: vec![
                LyricLine { time_secs: 0, text: "🌊 نهر دجلة يروي حضارات البداية...".to_string() },
            ],
        });
    }
    
    tracks
}

pub fn setup(app: &AppWindow) -> slint::Timer {
    let app_weak = app.as_weak();
    
    // Initial scan and load
    let tracks = scan_library();
    update_slint_tracks(app, &tracks);
    
    let tracks_cell = Arc::new(Mutex::new(tracks));
    let rng = Arc::new(Mutex::new(SimpleRng::new(12345)));
    
    // 1. Play Track Callback
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_play_track(move |index| {
        if let Some(app) = app_weak_clone.upgrade() {
            app.set_current_track_index(index);
            app.set_is_playing(true);
            app.set_current_time_secs(0);
            app.set_playback_progress(0.0);
            
            let guard = tracks_clone.lock().unwrap();
            if let Some(track) = guard.get(index as usize) {
                play_audio_file(&track.file_path, 0, track.duration_secs);
            }
        }
    });
    
    // 2. Toggle Playback Callback
    let app_weak_clone = app_weak.clone();
    app.on_toggle_playback(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let is_playing = app.get_is_playing();
            app.set_is_playing(!is_playing);
            
            if is_playing {
                pause_audio();
            } else {
                resume_audio();
            }
        }
    });
    
    // 3. Next Track Callback
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_next_track(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let count = app.get_tracks().row_count();
            if count > 0 {
                let mut next = app.get_current_track_index() + 1;
                if next >= count as i32 {
                    next = 0;
                }
                app.set_current_track_index(next);
                app.set_current_time_secs(0);
                app.set_playback_progress(0.0);
                app.set_is_playing(true);
                
                let guard = tracks_clone.lock().unwrap();
                if let Some(track) = guard.get(next as usize) {
                    play_audio_file(&track.file_path, 0, track.duration_secs);
                }
            }
        }
    });
    
    // 4. Previous Track Callback
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_prev_track(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let count = app.get_tracks().row_count();
            if count > 0 {
                let mut prev = app.get_current_track_index() - 1;
                if prev < 0 {
                    prev = count as i32 - 1;
                }
                app.set_current_track_index(prev);
                app.set_current_time_secs(0);
                app.set_playback_progress(0.0);
                app.set_is_playing(true);
                
                let guard = tracks_clone.lock().unwrap();
                if let Some(track) = guard.get(prev as usize) {
                    play_audio_file(&track.file_path, 0, track.duration_secs);
                }
            }
        }
    });
    
    // 5. Volume Control Callback
    app.on_set_volume(move |vol| {
        println!("[Raneem MP3] Volume set to: {:.2}", vol);
        let _ = Command::new("wpctl")
            .args(&["set-volume", "@DEFAULT_AUDIO_SINK@", &format!("{:.2}", vol)])
            .status();
    });
    
    // 6. Progress Slider Adjust Callback
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_set_progress(move |progress| {
        if let Some(app) = app_weak_clone.upgrade() {
            let idx = app.get_current_track_index();
            let guard = tracks_clone.lock().unwrap();
            if let Some(track) = guard.get(idx as usize) {
                let total_secs = track.duration_secs;
                let new_secs = (progress * total_secs as f32) as i32;
                app.set_current_time_secs(new_secs);
                app.set_playback_progress(progress);
                
                if app.get_is_playing() {
                    play_audio_file(&track.file_path, new_secs, total_secs);
                }
            }
        }
    });
    
    // Seek Forward Callback (10 seconds)
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_seek_forward(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let idx = app.get_current_track_index();
            let guard = tracks_clone.lock().unwrap();
            if let Some(track) = guard.get(idx as usize) {
                let total_secs = track.duration_secs;
                let mut secs = app.get_current_time_secs();
                secs = (secs + 10).clamp(0, total_secs);
                app.set_current_time_secs(secs);
                app.set_playback_progress(secs as f32 / total_secs as f32);
                
                if app.get_is_playing() {
                    play_audio_file(&track.file_path, secs, total_secs);
                }
            }
        }
    });

    // Seek Backward Callback (10 seconds)
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_seek_backward(move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let idx = app.get_current_track_index();
            let guard = tracks_clone.lock().unwrap();
            if let Some(track) = guard.get(idx as usize) {
                let total_secs = track.duration_secs;
                let mut secs = app.get_current_time_secs();
                secs = (secs - 10).clamp(0, total_secs);
                app.set_current_time_secs(secs);
                app.set_playback_progress(secs as f32 / total_secs as f32);
                
                if app.get_is_playing() {
                    play_audio_file(&track.file_path, secs, total_secs);
                }
            }
        }
    });
    
    // 7. Toggle Shuffle Callback
    app.on_toggle_shuffle(move || {
        println!("[Raneem MP3] Toggle Shuffle");
    });
    
    // 8. Toggle Loop Callback
    app.on_toggle_loop(move || {
        println!("[Raneem MP3] Toggle Loop");
    });
    
    // 9. Search query changed callback
    let app_weak_clone = app_weak.clone();
    app.on_search_changed(move |query| {
        println!("[Raneem MP3] Search query changed to: '{}'", query);
        if let Some(_app) = app_weak_clone.upgrade() {
            // Future feature: Filter the playlist model in Slint
        }
    });

    // 10. Add File Dialog Callback (Zenity)
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    app.on_add_file_clicked(move || {
        let app_weak_inner = app_weak_clone.clone();
        let tracks_inner = tracks_clone.clone();
        
        std::thread::spawn(move || {
            println!("[Raneem MP3] Launching native file dialog...");
            if let Ok(output) = Command::new("zenity")
                .args(&["--file-selection", "--file-filter=*.mp3"])
                .output() {
                
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        let path = Path::new(&path_str);
                        let meta = read_mp3_meta(path);
                        
                        let mut lrc_path = path.to_path_buf();
                        lrc_path.set_extension("lrc");
                        let lyrics = if lrc_path.exists() {
                            parse_lrc(&lrc_path)
                        } else {
                            Vec::new()
                        };
                        
                        let new_track = TrackData {
                            file_path: path_str.clone(),
                            title: meta.title,
                            artist: meta.artist,
                            duration_secs: meta.duration_secs,
                            lyrics,
                        };
                        
                        let duration_secs = meta.duration_secs;
                        let mut guard = tracks_inner.lock().unwrap();
                        guard.push(new_track);
                        let new_index = (guard.len() - 1) as i32;
                        
                        let guard_copy = guard.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(app) = app_weak_inner.upgrade() {
                                update_slint_tracks(&app, &guard_copy);
                                app.set_current_track_index(new_index);
                                app.set_is_playing(true);
                                app.set_current_time_secs(0);
                                app.set_playback_progress(0.0);
                                play_audio_file(&path_str, 0, duration_secs);
                            }
                        });
                    }
                }
            }
        });
    });
    
    // 11. Animation & Playback Timer Tick
    let timer = slint::Timer::default();
    let app_weak_clone = app_weak.clone();
    let tracks_clone = tracks_cell.clone();
    let rng_clone = rng.clone();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        if let Some(app) = app_weak_clone.upgrade() {
            let is_playing = app.get_is_playing();
            
            // A. Update ambient pulse glow and lyrics
            static mut TICK_COUNT: u32 = 0;
            static mut PULSE_TICK: u32 = 0;
            
            unsafe {
                PULSE_TICK += 1;
                if PULSE_TICK >= 8 {
                    PULSE_TICK = 0;
                    if is_playing {
                        let current_pulse = app.get_pulse_glow();
                        app.set_pulse_glow(if current_pulse > 0.0 { 0.0 } else { 0.25 });
                    } else {
                        app.set_pulse_glow(0.0);
                    }
                }
            }
            
            // B. Sync active lyrics
            let idx = app.get_current_track_index();
            let guard = tracks_clone.lock().unwrap();
            if let Some(track) = guard.get(idx as usize) {
                let current_sec = app.get_current_time_secs();
                let mut active_lyric = "🏺 عزف لوح موسيقي...".to_string();
                for line in &track.lyrics {
                    if line.time_secs <= current_sec {
                        active_lyric = line.text.clone();
                    } else {
                        break;
                    }
                }
                app.set_current_lyrics(active_lyric.into());
            }
            
            if is_playing {
                // C. Spin the vinyl disc (unless user is scratching it)
                if !app.get_is_vinyl_pressed() {
                    let mut current_angle = app.get_album_angle();
                    current_angle = (current_angle + 2.0) % 360.0;
                    app.set_album_angle(current_angle);
                }
                
                // D. Increment elapsed track playback time
                unsafe {
                    TICK_COUNT += 1;
                    if TICK_COUNT >= 10 {
                        TICK_COUNT = 0;
                        let mut secs = app.get_current_time_secs();
                        let idx = app.get_current_track_index();
                        if let Some(track) = guard.get(idx as usize) {
                            let total_secs = track.duration_secs;
                            if secs < total_secs {
                                secs += 1;
                                app.set_current_time_secs(secs);
                                app.set_playback_progress(secs as f32 / total_secs as f32);
                            } else {
                                // Track ended! If loop is on, restart. Else go next track.
                                if app.get_is_loop() {
                                    app.set_current_time_secs(0);
                                    app.set_playback_progress(0.0);
                                    play_audio_file(&track.file_path, 0, track.duration_secs);
                                } else {
                                    let count = guard.len();
                                    if count > 0 {
                                        let next = (idx + 1) % count as i32;
                                        app.set_current_track_index(next);
                                        app.set_current_time_secs(0);
                                        app.set_playback_progress(0.0);
                                        if let Some(next_track) = guard.get(next as usize) {
                                            play_audio_file(&next_track.file_path, 0, next_track.duration_secs);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // E. Animate visualizer bars with sine wave + pseudo-rng
                let mut visualizer_data: Vec<f32> = Vec::new();
                let mut r = rng_clone.lock().unwrap();
                let current_angle = app.get_album_angle();
                
                for idx in 0..20 {
                    let base_wave = (idx as f32 * 0.4 + (current_angle * 0.15)).sin().abs();
                    let random_noise = r.next_float() * 0.3;
                    let height = (base_wave * 0.7 + random_noise).clamp(0.05, 1.0);
                    visualizer_data.push(height);
                }
                
                let visualizer_model = rc_to_slint_model(visualizer_data);
                app.set_visualizer_heights(visualizer_model);
            }
        }
    });
    
    timer
}
