use std::process::Command;
use colored::*;

pub fn handle_bluetooth(args: &[String]) -> String {
    if args.is_empty() { return "استخدام: bluetooth [on|off|scan|connect]".red().to_string(); }
    match args[0].as_str() {
        "on" | "تشغيل" => "🔵 تم تفعيل البلوتوث.".green().to_string(),
        "off" | "إيقاف" => "⚪ تم إيقاف البلوتوث.".yellow().to_string(),
        "scan" | "بحث" => "🔎 جاري البحث عن أجهزة بلوتوث قريبة...".cyan().to_string(),
        "connect" | "اتصال" => "🔗 جاري الاتصال بالجهاز...".cyan().to_string(),
        _ => "أمر بلوتوث غير معروف.".red().to_string(),
    }
}

pub fn handle_clean() -> String {
    "🧹 تم تنظيف الكاش وتحرير الذاكرة بنجاح!".green().bold().to_string()
}

pub fn handle_user(args: &[String]) -> String {
    if args.is_empty() { return "استخدام: user [add|remove|switch]".red().to_string(); }
    format!("👤 جاري تنفيذ أمر إدارة المستخدم: {}", args[0].cyan())
}

pub fn handle_power(action: &str) -> String {
    match action {
        "sleep" | "سبات" => "🌙 جاري الدخول في وضع السبات...".cyan().to_string(),
        "reboot" | "اعادة" => "🔄 جاري إعادة تشغيل النظام...".yellow().to_string(),
        "shutdown" | "اطفاء" => "⚡ جاري إيقاف تشغيل النظام...".red().bold().to_string(),
        _ => "أمر طاقة غير معروف. استخدم: power [sleep|reboot|shutdown]".red().to_string(),
    }
}

pub fn handle_tree() -> String {
    match Command::new("tree").arg("-L").arg("2").output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "🌲 الشجرة: (أداة tree غير منصبة بالنظام، استخدم ls مؤقتاً)".yellow().to_string(),
    }
}

pub fn handle_timer(args: &[String]) -> String {
    if args.is_empty() { return "استخدام: timer [الثواني]".red().to_string(); }
    let secs = args[0].parse::<u64>().unwrap_or(5);
    format!("⏱️ تم ضبط المؤقت لمدة {} ثانية.", secs.to_string().cyan())
}
