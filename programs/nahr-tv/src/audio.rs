use std::thread;
use std::path::Path;
use std::process::Command;

pub fn play_navigation_sound() {
    thread::spawn(move || {
        let paths = vec![
            "ui/assets/audio.mp3",
            "programs/nahr-tv/ui/assets/audio.mp3",
            "ui/firmament/assets/audio.mp3",
            "/usr/bin/firmament/assets/audio.mp3",
            "sumer-ui/ui/firmament/assets/audio.mp3",
            "/usr/share/sumer/audio.mp3",
            "programs/sumer-ui/ui/firmament/assets/audio.mp3",
        ];
        let mut sound_path = "ui/assets/audio.mp3".to_string();
        for p in &paths {
            if Path::new(p).exists() {
                sound_path = p.to_string();
                break;
            }
        }
        
        if Command::new("gst-play-1.0").arg(&sound_path).spawn().is_ok() { return; }
        if Command::new("mpg123").arg("-q").arg(&sound_path).spawn().is_ok() { return; }
        if Command::new("pw-play").arg(&sound_path).spawn().is_ok() { return; }
        if Command::new("paplay").arg(&sound_path).spawn().is_ok() { return; }
        if Command::new("ffplay").args(&["-nodisp", "-autoexit", "-loglevel", "quiet", &sound_path]).spawn().is_ok() { return; }
    });
}
