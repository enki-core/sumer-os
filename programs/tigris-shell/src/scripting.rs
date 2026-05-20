use colored::*;

pub fn run_script(file_path: &str) -> String {
    if !file_path.ends_with(".tigris") {
        return format!("{}: الملف يجب أن ينتهي بالامتداد .tigris", "خطأ".red().bold());
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            let mut results = Vec::new();
            results.push(format!("🎬 جاري تشغيل سكربت دجلة: {}", file_path));
            results.push("----------------------------------------".to_string());
            
            for (idx, line) in content.lines().enumerate() {
                let line_trimmed = line.trim();
                if line_trimmed.is_empty() || line_trimmed.starts_with('#') {
                    continue; // Skip comments and empty lines
                }
                
                results.push(format!("❯ [{}] تشغيل: {}", idx + 1, line_trimmed));
                
                let parts: Vec<&str> = line_trimmed.split_whitespace().collect();
                let command = parts[0];
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                
                // We dispatch and run it through the unified engine!
                let output = crate::shell::execute_internal_dispatch(command, &args);
                results.push(output);
                results.push("".to_string());
            }
            
            results.push("----------------------------------------".to_string());
            results.push("🏁 تم الانتهاء من تشغيل السكربت بنجاح!".green().bold().to_string());
            results.join("\n")
        }
        Err(e) => {
            format!("{}: فشل في قراءة الملف '{}' - {}", "خطأ".red().bold(), file_path, e)
        }
    }
}
