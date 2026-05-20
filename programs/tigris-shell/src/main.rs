mod shell;
mod commands;
mod scripting;

use std::env;

slint::include_modules!();

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "--cli" || first_arg == "-c" {
            // Interactive terminal loop mode (CLI)
            shell::run_interactive_shell();
        } else {
            // Direct execution mode for GUI API calls / CLI scripting calls
            let command = &args[1];
            let cmd_args = &args[2..];
            shell::execute_direct_command(command, cmd_args);
        }
    } else {
        // Launch GUI by default if no arguments are passed
        if let Err(e) = run_gui() {
            eprintln!("Failing to launch Tigris GUI: {:?}", e);
            // Fallback to interactive shell if GUI fails (e.g. no X11/Wayland display)
            shell::run_interactive_shell();
        }
    }
}

fn run_gui() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;
    
    window.on_execute_command(move |input| {
        let input = input.trim();
        if input.is_empty() {
            return "".into();
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        if command == "exit" || command == "خروج" {
            slint::quit_event_loop().ok();
            return "جاري إغلاق شيل دجلة...".into();
        }
        
        let output = shell::execute_internal_dispatch(command, &args);
        
        // Remove ANSI color escape codes from the output since Slint Text rendering does not parse them
        let clean_output = strip_ansi_escapes(&output);
        clean_output.into()
    });
    
    window.run()
}

fn strip_ansi_escapes(input: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\x1B' {
            in_escape = true;
            if chars.peek() == Some(&'[') {
                chars.next(); // Skip '['
            }
        } else if in_escape {
            // ANSI escape sequences typically end with a letter (e.g., 'm', 'J', 'H')
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }
    
    result
}
