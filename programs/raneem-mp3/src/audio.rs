use std::io::{Read, Seek, SeekFrom};
use std::thread;
use std::path::Path;
use std::process::{Command, Child};
use std::sync::Mutex;
use std::fs::{self, File};

// Simple LCG pseudo-random generator
pub struct SimpleRng {
    seed: u32,
}

impl SimpleRng {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
    
    pub fn next_float(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.seed / 65536) % 32768) as f32 / 32768.0
    }
}

// Global active player process
static ACTIVE_CHILD: Mutex<Option<Child>> = Mutex::new(None);

pub struct Mp3Meta {
    pub title: String,
    pub artist: String,
    pub duration_secs: i32,
}

#[derive(Clone)]
pub struct LyricLine {
    pub time_secs: i32,
    pub text: String,
}

// Custom ID3 v2 reader (pure Rust, no dependencies)
pub fn read_mp3_meta(path: &Path) -> Mp3Meta {
    let filename = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let mut meta = Mp3Meta {
        title: filename,
        artist: "فنان مجهول".to_string(),
        duration_secs: 225, // default 3:45
    };
    
    // Read file size to estimate duration
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        // Assuming ~192 kbps average (24 KB/s)
        meta.duration_secs = (size / 24000).clamp(1, 7200) as i32;
    }
    
    if let Ok(mut file) = File::open(path) {
        let mut header = [0u8; 10];
        if file.read_exact(&mut header).is_ok() {
            if &header[0..3] == b"ID3" {
                let tag_size = ((header[6] as usize) << 21) |
                               ((header[7] as usize) << 14) |
                               ((header[8] as usize) << 7) |
                               (header[9] as usize);
                
                let mut tag_data = vec![0u8; tag_size];
                if file.read_exact(&mut tag_data).is_ok() {
                    let mut pos = 0;
                    while pos + 10 < tag_size {
                        let frame_id = &tag_data[pos..pos+4];
                        let frame_size = ((tag_data[pos+4] as usize) << 24) |
                                             ((tag_data[pos+5] as usize) << 16) |
                                             ((tag_data[pos+6] as usize) << 8) |
                                             (tag_data[pos+7] as usize);
                        
                        pos += 10;
                        if pos + frame_size > tag_size { break; }
                        
                        let frame_content = &tag_data[pos..pos+frame_size];
                        if frame_id == b"TIT2" {
                            if let Some(text) = parse_id3_text(frame_content) {
                                meta.title = text;
                            }
                        } else if frame_id == b"TPE1" {
                            if let Some(text) = parse_id3_text(frame_content) {
                                meta.artist = text;
                            }
                        }
                        pos += frame_size;
                    }
                }
            }
        }
    }
    meta
}

fn parse_id3_text(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    let encoding = data[0];
    let text_data = &data[1..];
    
    // Clean string from null bytes and whitespace
    let clean = |s: String| {
        s.replace('\0', "").trim().to_string()
    };
    
    match encoding {
        0 => { // ISO-8859-1
            let s: String = text_data.iter().map(|&c| c as char).collect();
            Some(clean(s))
        }
        3 => { // UTF-8
            String::from_utf8(text_data.to_vec()).ok().map(clean)
        }
        _ => {
            String::from_utf8(text_data.to_vec()).ok().map(clean)
        }
    }
}

// Synced lyrics LRC parser
pub fn parse_lrc(path: &Path) -> Vec<LyricLine> {
    let mut lines = Vec::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if line.starts_with('[') && line.len() > 10 {
                let parts: Vec<&str> = line.splitn(2, ']').collect();
                if parts.len() == 2 {
                    let time_part = &parts[0][1..];
                    let text = parts[1].trim().to_string();
                    if let Some(secs) = parse_lrc_time(time_part) {
                        lines.push(LyricLine { time_secs: secs, text });
                    }
                }
            }
        }
    }
    lines.sort_by_key(|l| l.time_secs);
    lines
}

fn parse_lrc_time(time_str: &str) -> Option<i32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        let mins = parts[0].parse::<i32>().ok()?;
        let secs_parts: Vec<&str> = parts[1].split('.').collect();
        let secs = secs_parts[0].parse::<i32>().ok()?;
        return Some(mins * 60 + secs);
    }
    None
}
// Audio Control Actions
pub fn play_audio_file(file_path: &str, start_secs: i32, total_secs: i32) {
    let path = file_path.to_string();
    thread::spawn(move || {
        // Kill existing player
        stop_audio();
        
        let mut final_path = path.clone();
        
        if start_secs > 0 && total_secs > 0 {
            if let Ok(metadata) = fs::metadata(&path) {
                let size = metadata.len() as usize;
                let pct = (start_secs as f32 / total_secs as f32).clamp(0.0, 1.0);
                let start_bytes = ((pct * size as f32) as usize) & !3;
                
                if start_bytes > 0 {
                    if let Ok(mut file) = File::open(&path) {
                        let _ = file.seek(SeekFrom::Start(start_bytes as u64));
                        let mut buffer = Vec::new();
                        if file.read_to_end(&mut buffer).is_ok() {
                            let _ = fs::create_dir_all("target");
                            let temp_path = "target/seek_temp.mp3";
                            if fs::write(temp_path, &buffer).is_ok() {
                                final_path = temp_path.to_string();
                            }
                        }
                    }
                }
            }
        }
        
        let mut guard = ACTIVE_CHILD.lock().unwrap();
        println!("[Raneem Audio] Playing native audio: {} (from offset: {}s)", final_path, start_secs);
        if let Ok(child) = Command::new("pw-play").arg(&final_path).spawn() {
            *guard = Some(child);
        }
    });
}

pub fn pause_audio() {
    let guard = ACTIVE_CHILD.lock().unwrap();
    if let Some(child) = guard.as_ref() {
        let pid = child.id();
        println!("[Raneem Audio] Sending SIGSTOP to process {}", pid);
        let _ = Command::new("kill").args(&["-STOP", &pid.to_string()]).status();
    }
}

pub fn resume_audio() {
    let guard = ACTIVE_CHILD.lock().unwrap();
    if let Some(child) = guard.as_ref() {
        let pid = child.id();
        println!("[Raneem Audio] Sending SIGCONT to process {}", pid);
        let _ = Command::new("kill").args(&["-CONT", &pid.to_string()]).status();
    }
}

pub fn stop_audio() {
    let mut guard = ACTIVE_CHILD.lock().unwrap();
    if let Some(mut child) = guard.take() {
        println!("[Raneem Audio] Killing active audio process {}", child.id());
        let _ = child.kill();
    }
}
