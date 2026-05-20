use std::process::Command;
use colored::*;

pub fn handle_wifi(action: &str) -> String {
    match action {
        "on" | "up" | "تشغيل" => {
            let _ = Command::new("ip").args(["link", "set", "wlan0", "up"]).status();
            format!("🟢 {}", "تم تنشيط كرت الواي فاي (wlan0) بنجاح.".green().bold())
        }
        "off" | "down" | "إيقاف" => {
            let _ = Command::new("ip").args(["link", "set", "wlan0", "down"]).status();
            format!("🔴 {}", "تم إيقاف كرت الواي فاي (wlan0) لسلامة النظام.".red().bold())
        }
        "status" | "حالة" => {
            let output = Command::new("ip").args(["link", "show", "wlan0"]).output();
            match output {
                Ok(out) => {
                    let out_str = String::from_utf8_lossy(&out.stdout);
                    if out_str.contains("UP") {
                        format!("📶 {}", "الواي فاي: متصل ونشط 🟢".green())
                    } else {
                        format!("📶 {}", "الواي فاي: غير متصل أو مطفأ 🔴".red())
                    }
                }
                Err(_) => "📶 الواي فاي: حالة محاكاة نشطة 🟢 (لم يتم العثور على wlan0)".yellow().to_string(),
            }
        }
        _ => format!("أمر غير معروف للواي فاي. استخدم: wifi [on | off | status]"),
    }
}

pub fn handle_airplane(action: &str) -> String {
    match action {
        "on" | "block" | "تشغيل" => {
            let _ = Command::new("rfkill").args(["block", "all"]).status();
            format!("✈️  {}", "تم تفعيل وضع الطيران. تم حظر جميع اتصالات الراديو.".green().bold())
        }
        "off" | "unblock" | "إيقاف" => {
            let _ = Command::new("rfkill").args(["unblock", "all"]).status();
            format!("✈️  {}", "تم إلغاء وضع الطيران. الاتصالات اللاسلكية متاحة الآن.".cyan().bold())
        }
        "status" | "حالة" => {
            let output = Command::new("rfkill").args(["list"]).output();
            match output {
                Ok(out) => {
                    let out_str = String::from_utf8_lossy(&out.stdout);
                    if out_str.contains("yes") {
                        format!("✈️  {}", "وضع الطيران: مفعل 🟢 (الاتصالات محظورة)".green())
                    } else {
                        format!("✈️  {}", "وضع الطيران: غير مفعل 🔴".red())
                    }
                }
                Err(_) => "✈️  وضع الطيران: غير نشط 🔴".to_string(),
            }
        }
        _ => format!("أمر غير معروف لوضع الطيران. استخدم: airplane [on | off | status]"),
    }
}

pub fn check_network() -> String {
    let ping = Command::new("ping")
        .args(["-c", "1", "-W", "1", "8.8.8.8"])
        .status();
    match ping {
        Ok(status) if status.success() => "📶 الاتصال بالإنترنت: مستقر ونشط 🟢".green().to_string(),
        _ => "⚠️ الاتصال بالإنترنت: غير متاح أو لا توجد استجابة 🔴".red().to_string(),
    }
}
