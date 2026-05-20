use colored::*;

pub fn handle_app(args: &[String]) -> String {
    if args.len() < 2 {
        return "استخدام: app [launch|kill] [اسم_التطبيق]".red().to_string();
    }
    let action = &args[0];
    let app_name = &args[1];

    match action.as_str() {
        "launch" | "تشغيل" => format!("🚀 جاري تشغيل التطبيق: {}...", app_name.cyan()),
        "kill" | "اغلاق" => format!("🛑 تم إغلاق التطبيق: {}", app_name.red()),
        _ => "أمر تطبيق غير معروف.".red().to_string(),
    }
}

pub fn run_calc(args: &[String]) -> String {
    // يستدعي مجلد sixteen-calc
    let cmd = format!("../sixteen-calc/run {}", args.join(" "));
    format!("🧮 استدعاء الحاسبة المتقدمة من (sixteen-calc)...\nالأمر المجهز: {}", cmd.cyan())
}

pub fn run_notepad(args: &[String]) -> String {
    // يستدعي مجلد alwah-notepad
    let cmd = format!("../alwah-notepad/run {}", args.join(" "));
    format!("📝 استدعاء محرر النصوص من (alwah-notepad)...\nالأمر المجهز: {}", cmd.cyan())
}

pub fn run_player(args: &[String]) -> String {
    // يستدعي مجلد raneem-mp3
    let cmd = format!("../raneem-mp3/run {}", args.join(" "));
    format!("🎵 استدعاء مشغل الصوتيات من (raneem-mp3)...\nالأمر المجهز: {}", cmd.cyan())
}

pub fn run_video(args: &[String]) -> String {
    // يستدعي مجلد afak-mp4
    let cmd = format!("../afak-mp4/run {}", args.join(" "));
    format!("🎬 استدعاء مشغل الفيديو من (afak-mp4)...\nالأمر المجهز: {}", cmd.cyan())
}

pub fn run_browser(args: &[String]) -> String {
    // يستدعي مجلد orok-browser
    let cmd = format!("../orok-browser/run {}", args.join(" "));
    format!("🌐 استدعاء المتصفح من (orok-browser)...\nالأمر المجهز: {}", cmd.cyan())
}

pub fn run_store(args: &[String]) -> String {
    // يستدعي مجلد zaqura-store
    let cmd = format!("../zaqura-store/run {}", args.join(" "));
    format!("🛍️ استدعاء متجر التطبيقات من (zaqura-store)...\nالأمر المجهز: {}", cmd.cyan())
}
