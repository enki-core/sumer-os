use std::io::{self, Write};
use std::process::Command;
use colored::*;

use crate::commands::{network, system, resources, apps, system_ext, security};
use crate::scripting;

pub fn run_interactive_shell() {
    display_welcome();

    loop {
        print!("{} ", "tigris ❯".cyan().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("خطأ في قراءة الأمر.");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        if command == "exit" || command == "خروج" {
            println!("{}", "وداعاً! تم الخروج من شيل دجلة.".yellow());
            break;
        } else if command == "clear" || command == "مسح" {
            print!("\x1B[2J\x1B[H");
            continue;
        }

        let output = execute_internal_dispatch(command, &args);
        println!("{}", output);
    }
}

pub fn execute_direct_command(command: &str, args: &[String]) {
    let output = execute_internal_dispatch(command, args);
    print!("{}", output);
}

pub fn execute_internal_dispatch(command: &str, args: &[String]) -> String {
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match command {
        // --- الأساسيات ---
        "help" | "مساعدة" | "--help" | "-h" => get_help_text(),
        "about" | "حول" => "Tigris: شيل دجلة هو محرك العتاد والأوامر التفاعلي لنظام Sumer OS.\nتمت إعادة تصميمه بالكامل بلغة Rust الصرفة ليكون فائق الخفة والسرعة.".to_string(),
        "sysinfo" | "نظام" => system::get_sysinfo(),
        "neofetch" | "شعار" => system::get_neofetch(),
        
        // --- التطبيقات (Apps) ---
        "app" | "تطبيق" => apps::handle_app(args),
        "calc" | "حاسبة" => apps::run_calc(args),
        "view" | "notepad" | "مفكرة" => apps::run_notepad(args),
        "play" | "صوتيات" => apps::run_player(args),
        "video" | "فيديو" => apps::run_video(args),
        "browser" | "متصفح" => apps::run_browser(args),
        "store" | "متجر" => apps::run_store(args),

        // --- النظام الممتد (System Ext) ---
        "bluetooth" | "بلوتوث" => system_ext::handle_bluetooth(args),
        "clean" | "تنظيف" => system_ext::handle_clean(),
        "user" | "مستخدم" => system_ext::handle_user(args),
        "power" | "طاقة" => system_ext::handle_power(args.get(0).map(|s| s.as_str()).unwrap_or("")),
        "tree" | "شجرة" | "شجره" => system_ext::handle_tree(),
        "timer" | "مؤقت" => system_ext::handle_timer(args),
        "upd" | "تحديث" => "🔄 جاري تحديث النظام (Sumer Update)...".cyan().to_string(),

        // --- الشبكات والأمان (Security & Networking) ---
        "wifi" | "وايفاي" => {
            let status = if !args.is_empty() { &args[0] } else { "status" };
            network::handle_wifi(status)
        }
        "airplane" | "طيران" => {
            let status = if !args.is_empty() { &args[0] } else { "status" };
            network::handle_airplane(status)
        }
        "network" | "شبكة" => network::check_network(),
        "ports" | "منافذ" => security::handle_ports(),
        "passgen" | "توليد" => security::handle_passgen(args),
        "speedtest" | "سرعة" => security::handle_speedtest(),
        "fw" | "جدار" => security::handle_firewall(args),
        "share" | "مشاركة" => security::handle_share(args),

        // --- الموارد والأدوات (Resources & Battery) ---
        "battery" | "بطارية" => system::get_battery(),
        "resources" | "موارد" => resources::get_resources_info(),
        "run" | "تشغيل" => {
            if args.is_empty() {
                format!("{}: يرجى تحديد مسار ملف السكربت .tigris", "خطأ".red().bold())
            } else {
                scripting::run_script(&args[0])
            }
        }
        _ => {
            match Command::new(command).args(&args_str).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if output.status.success() {
                        stdout.into_owned()
                    } else {
                        format!("{}\n{}", stderr.red(), stdout)
                    }
                }
                Err(_) => format!("{}: الأمر '{}' غير معروف في دجلة أو النظام.", "خطأ".red().bold(), command),
            }
        }
    }
}

fn display_welcome() {
    println!("{}", "========================================".blue());
    println!("{}", "      Tigris Shell | شيل دجلة v1.5.0    ".bold().yellow());
    println!("{}", "     أداة النظام التفاعلية الرسمية لـ Sumer OS  ".bold().white());
    println!("{}", "========================================".blue());
    println!("أكتب {} أو {} لعرض الأوامر المتوفرة.", "help".cyan(), "مساعدة".cyan());
    println!();
}

fn get_help_text() -> String {
    let mut help = String::new();
    help.push_str(&format!("{}\n{}\n", "شيل دجلة - أداة النظام الموحدة".bold().yellow(), "الأوامر المتوفرة:".green()));
    
    let commands = vec![
        ("help / مساعدة", "عرض هذه المساعدة"),
        ("sysinfo / نظام", "معلومات نظام التشغيل"),
        ("app / تطبيق", "إدارة التطبيقات [launch|kill]"),
        ("calc / حاسبة", "تشغيل الحاسبة المتقدمة"),
        ("view / مفكرة", "عرض وتعديل النصوص والمقررات"),
        ("play / صوتيات", "مشغل الصوتيات الخلفي"),
        ("video / فيديو", "مشغل مرئيات الفيديو"),
        ("browser / متصفح", "متصفح الويب الذكي"),
        ("store / متجر", "متجر برمجيات زقورة"),
        ("bluetooth / بلوتوث", "التحكم بالبلوتوث [on|off|scan]"),
        ("clean / تنظيف", "تنظيف النظام وتحرير الذاكرة"),
        ("power / طاقة", "إدارة الطاقة [sleep|reboot]"),
        ("tree / شجرة", "عرض هيكل الملفات كشجرة"),
        ("timer / مؤقت", "ضبط مؤقت ذكي لتنفيذ المهام"),
        ("upd / تحديث", "تحديث النظام السريع"),
        ("wifi / وايفاي", "التحكم بالواي فاي [on|off|status]"),
        ("ports / منافذ", "مراقبة المنافذ النشطة"),
        ("passgen / توليد", "توليد كلمة مرور آمنة"),
        ("speedtest / سرعة", "فحص سرعة الإنترنت"),
        ("fw / جدار", "التحكم بجدار الحماية"),
        ("share / مشاركة", "مشاركة الملفات السريعة"),
        ("resources / موارد", "مراقبة المعالج والذاكرة (RAM)"),
        ("run / تشغيل", "تشغيل سكربت (.tigris) مخصص"),
    ];

    for (cmd, desc) in commands {
        help.push_str(&format!("  {:<25} - {}\n", cmd, desc));
    }
    help
}
