use std::path::Path;
use std::process::Command;

pub fn launch_file(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        return;
    }
    
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
        
    let process = match ext.as_str() {
        // Text files: Alwah Notepad
        "txt" | "md" | "slint" | "rs" | "toml" | "json" | "css" | "sh" | "xml" | "html" | "conf" | "cfg" => {
            let alwah_path = "/home/debian/Desktop/Sumer-OS-Portable/programs/alwah-notepad/alwah-notepad";
            if Path::new(alwah_path).exists() {
                Command::new(alwah_path).arg(file_path).spawn()
            } else {
                Command::new("xdg-open").arg(file_path).spawn()
            }
        }
        // Audio files: Raneem MP3
        "mp3" | "wav" | "ogg" | "flac" | "m4a" => {
            let raneem_path = "/home/debian/Desktop/Sumer-OS-Portable/programs/raneem-mp3/target/release/raneem-mp3";
            let raneem_path_alt = "/home/debian/Desktop/Sumer-OS-Portable/programs/raneem-mp3/raneem-mp3";
            let raneem_bin = if Path::new(raneem_path).exists() {
                Some(raneem_path)
            } else if Path::new(raneem_path_alt).exists() {
                Some(raneem_path_alt)
            } else {
                None
            };
            
            if let Some(bin) = raneem_bin {
                Command::new(bin).arg(file_path).spawn()
            } else {
                Command::new("xdg-open").arg(file_path).spawn()
            }
        }
        // Video files: Afak MP4
        "mp4" | "mkv" | "avi" | "mov" | "webm" => {
            let afak_path = "/home/debian/Desktop/Sumer-OS-Portable/programs/afak-mp4/target/release/afak-mp4";
            let afak_path_alt = "/home/debian/Desktop/Sumer-OS-Portable/programs/afak-mp4/afak-mp4";
            let afak_bin = if Path::new(afak_path).exists() {
                Some(afak_path)
            } else if Path::new(afak_path_alt).exists() {
                Some(afak_path_alt)
            } else {
                None
            };
            
            if let Some(bin) = afak_bin {
                Command::new(bin).arg(file_path).spawn()
            } else {
                Command::new("xdg-open").arg(file_path).spawn()
            }
        }
        _ => {
            Command::new("xdg-open").arg(file_path).spawn()
        }
    };
    
    if let Err(e) = process {
        eprintln!("Failed to launch file {}: {}", file_path, e);
    }
}
