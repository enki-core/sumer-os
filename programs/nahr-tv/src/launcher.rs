use std::path::Path;
use std::process::Command;

pub fn launch_app(app_name: &str) {
    println!("Centralized TV Launcher: launching {}", app_name);

    let binary_path = match app_name {
        "Store" | "zaqura-store" => "/usr/bin/zaqura-store",
        "Files" | "📂" => "/usr/bin/nautilus", // Sumer Files explorer
        "Console" | "💻" | "شيل دجلة" => "/usr/bin/tigris-shell",
        "Browser" | "orok-browser" => "/programs/orok-browser/run",
        "Shutdown" | "إغلاق" => {
            Command::new("poweroff").spawn().ok();
            std::process::exit(0);
        }
        "MediaPlayer" => "/programs/orok-media/run",
        "RetroGames" => "/programs/retro-games/run",
        "TextEditor" => "/programs/alwah-notepad/run",
        "SysMonitor" => "/programs/sumer-monitor/run",
        "MusicPlayer" => "/programs/sumer-music/run",
        "Docker" => "/programs/docker-manager/run",
        "Calculator" => "/programs/sumer-calc/run",
        "Network" => "/programs/network-analyzer/run",
        "Recorder" => "/programs/screen-recorder/run",
        "AIAssistant" => "/programs/sumer-ai/run",
        _ => return,
    };

    let mut actual_path = binary_path.to_string();
    
    // Development fallback paths
    if !Path::new(&actual_path).exists() {
        actual_path = match app_name {
            "Store" => "../zaqura-store/target-alpine/release/zaqura-store".to_string(),
            "Console" => "../tigris-shell/target-alpine/release/tigris-shell".to_string(),
            "TextEditor" => "../alwah-notepad/target-alpine/release/alwah-notepad".to_string(),
            _ => actual_path,
        };
    }

    if Path::new(&actual_path).exists() {
        Command::new(&actual_path).spawn().ok();
    } else {
        // Fallback to searching in PATH directly if it's a common Linux utility
        let cmd = match app_name {
            "Files" => Some("nautilus"),
            "Console" => Some("xterm"),
            "Browser" => Some("firefox"),
            _ => None,
        };
        if let Some(c) = cmd {
            Command::new(c).spawn().ok();
        }
    }
}
