use std::process::Command;
use colored::*;

pub fn get_sysinfo() -> String {
    let kernel = Command::new("uname").arg("-r").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "6.6.139-lts".to_string());

    let hostname = Command::new("hostname").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "sumer-os".to_string());

    let uptime = Command::new("uptime").arg("-p").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "up 4520 years".to_string());

    format!(
        "{}\n{}\n{}\n{}\n{}",
        format!("📋   نظام التشغيل: {}", "Sumer OS v1.5.0 Portable (Native Linux Edition)".cyan()).bold(),
        format!("🧬   نواة النظام: {}", kernel.yellow()),
        format!("💻   اسم المضيف: {}", hostname.green()),
        format!("⏳   مدة التشغيل: {}", uptime.magenta()),
        format!("🛠️   البيئة الرسومية: {}", "Slint + Wayland (FemtoVG KMS)".bold())
    )
}

pub fn get_neofetch() -> String {
    let hostname = "debian@sumer-os".cyan().bold();
    let line = "--------------".white();
    let os = format!("OS: {}", "Sumer OS v1.5.0 x86_64".yellow());
    let kernel = format!("Kernel: {}", "6.6.139-lts-sumer".green());
    let uptime = format!("Uptime: {}", "4520 years, 3 months".magenta());
    let shell = format!("Shell: {}", "Tigris Shell (Rust CLI)".blue());
    let wm = format!("WM: {}", "Cage (Wayland compositor)".red());
    let ui = format!("UI Engine: {}", "Slint GUI Framework".cyan());
    
    format!(
        r#"
      /\        {}
     /  \       {}
    /____\      {}
   /\    /\     {}
  /  \  /  \    {}
 /____\/____\   {}
                {}
                {}
"#,
        hostname, line, os, kernel, uptime, shell, wm, ui
    )
}

pub fn get_battery() -> String {
    match std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
        Ok(cap) => {
            let status = std::fs::read_to_string("/sys/class/power_supply/BAT0/status")
                .unwrap_or_else(|_| "Discharging".to_string());
            format!("🔋 حالة البطارية: {}% [{}]", cap.trim(), status.trim())
        }
        Err(_) => "🔋 حالة البطارية: 100% [كاملة / متصل بالشاحن] (بيئة افتراضية)".cyan().to_string(),
    }
}
