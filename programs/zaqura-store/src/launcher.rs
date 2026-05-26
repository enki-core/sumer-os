use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve `programs/<folder>` for a store app id (handles leading spaces in folder names).
fn resolve_program_dir(app_id: &str) -> Option<PathBuf> {
    let programs = Path::new("../");
    let exact = programs.join(app_id);
    if exact.is_dir() {
        return Some(exact);
    }
    let trimmed = app_id.trim();
    if trimmed != app_id {
        let p = programs.join(trimmed);
        if p.is_dir() {
            return Some(p);
        }
    }
    let Ok(entries) = fs::read_dir(programs) else {
        return None;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.trim() == trimmed {
            return Some(entry.path());
        }
    }
    None
}

fn cargo_package_name(project_dir: &Path) -> String {
    let toml = project_dir.join("Cargo.toml");
    let Ok(content) = fs::read_to_string(toml) else {
        return project_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .trim()
            .to_string();
    };
    for line in content.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("name = \"").and_then(|s| s.strip_suffix('"')) {
            return name.to_string();
        }
    }
    project_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .trim()
        .to_string()
}

pub fn launch_application(app_id: &str) {
    println!("[+] Zaqura Store launching app: {}", app_id);
    
    let command_to_run = match app_id {
        "telegram-desktop" | "chromium" | "vlc" | "discord" | "gimp" | "code-oss" => {
            let binary_name = match app_id {
                "chromium" => "chromium-browser",
                other => other,
            };
            println!("[*] Launching external system app: {}", binary_name);
            Some(Command::new(binary_name))
        }
        _ => {
            let pkg = app_id.trim();
            let sys_path = format!("/usr/bin/{}", pkg);

            if let Some(project_dir) = resolve_program_dir(app_id) {
                let bin_name = cargo_package_name(&project_dir);
                let alpine_path = project_dir.join("target-alpine/release").join(&bin_name);
                let root_path = project_dir.join(&bin_name);
                let debug_path = project_dir.join("target/debug").join(&bin_name);

                if Path::new(&sys_path).exists() {
                    println!("[*] Found Sumer OS system path: {:?}", sys_path);
                    Some(Command::new(sys_path))
                } else if alpine_path.exists() {
                    println!("[*] Found Alpine release path: {:?}", alpine_path);
                    Some(Command::new(alpine_path))
                } else if root_path.exists() {
                    println!("[*] Found local copied path: {:?}", root_path);
                    Some(Command::new(root_path))
                } else if debug_path.exists() {
                    println!("[*] Found debug path: {:?}", debug_path);
                    Some(Command::new(debug_path))
                } else {
                    println!(
                        "[*] Found Cargo project dir: {:?}. Spawning 'cargo run'...",
                        project_dir
                    );
                    let mut cmd = Command::new("cargo");
                    cmd.arg("run").current_dir(project_dir);
                    Some(cmd)
                }
            } else if Path::new(&sys_path).exists() {
                println!("[*] Found Sumer OS system path: {:?}", sys_path);
                Some(Command::new(sys_path))
            } else {
                None
            }
        }
    };
    
    if let Some(mut cmd) = command_to_run {
        match cmd.spawn() {
            Ok(_) => println!("[+] Successfully spawned process for {}", app_id),
            Err(e) => eprintln!("[-] Failed to spawn process for {}: {}", app_id, e),
        }
    } else {
        eprintln!("[-] Could not locate any executable or project folder for {}", app_id);
    }
}
