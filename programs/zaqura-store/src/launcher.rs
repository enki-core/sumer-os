use std::path::Path;
use std::process::Command;

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
            let sys_path = format!("/usr/bin/{}", app_id);
            let alpine_path = format!("../{}/target-alpine/release/{}", app_id, app_id);
            let root_path = format!("../{}/{}", app_id, app_id);
            let debug_path = format!("../{}/target/debug/{}", app_id, app_id);
            
            if Path::new(&sys_path).exists() {
                println!("[*] Found Sumer OS system path: {:?}", sys_path);
                Some(Command::new(sys_path))
            } else if Path::new(&alpine_path).exists() {
                println!("[*] Found Alpine release path: {:?}", alpine_path);
                Some(Command::new(alpine_path))
            } else if Path::new(&root_path).exists() {
                println!("[*] Found local copied path: {:?}", root_path);
                Some(Command::new(root_path))
            } else if Path::new(&debug_path).exists() {
                println!("[*] Found debug path: {:?}", debug_path);
                Some(Command::new(debug_path))
            } else {
                let project_dir = format!("../{}", app_id);
                if Path::new(&project_dir).exists() {
                    println!("[*] Found Cargo project dir: {:?}. Spawning 'cargo run'...", project_dir);
                    let mut cmd = Command::new("cargo");
                    cmd.arg("run").current_dir(project_dir);
                    Some(cmd)
                } else {
                    None
                }
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
