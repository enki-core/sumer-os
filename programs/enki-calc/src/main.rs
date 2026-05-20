slint::include_modules!();

mod variables;
mod history;
mod math;

use variables::Variables;
use history::History;
use math::MathEvaluator;

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    // Initialize modules
    let vars = Variables::new();
    let hist = History::new();

    let app_weak = app.as_weak();
    app.on_button_pressed(move |btn| {
        if let Some(app) = app_weak.upgrade() {
            let mut current = app.get_expression().to_string();
            app.set_syntax_error("".into());
            
            if btn == "()" {
                let open = current.chars().filter(|c| *c == '(').count();
                let close = current.chars().filter(|c| *c == ')').count();
                if open > close && !current.ends_with('(') {
                    current.push(')');
                } else {
                    current.push('(');
                }
            } else {
                current.push_str(btn.as_str());
            }
            app.set_expression(current.into());
        }
    });

    let app_weak = app.as_weak();
    app.on_clear(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_expression("".into());
            app.set_result("0".into());
            app.set_syntax_error("".into());
        }
    });

    let app_weak = app.as_weak();
    app.on_backspace(move || {
        if let Some(app) = app_weak.upgrade() {
            let mut current = app.get_expression().to_string();
            app.set_syntax_error("".into());
            if !current.is_empty() {
                if current.ends_with("sin(") || current.ends_with("cos(") || current.ends_with("tan(") 
                    || current.ends_with("sec(") || current.ends_with("csc(") || current.ends_with("cot(") {
                    current.truncate(current.len() - 4);
                } else if current.ends_with("sqrt(") {
                    current.truncate(current.len() - 5);
                } else if current.ends_with("pi") {
                    current.truncate(current.len() - 2);
                } else {
                    current.pop();
                }
                app.set_expression(current.into());
            }
        }
    });

    let app_weak = app.as_weak();
    let vars_calc = vars.clone();
    let hist_calc = hist.clone();
    app.on_calculate(move || {
        if let Some(app) = app_weak.upgrade() {
            let raw_expr = app.get_expression().to_string();
            app.set_syntax_error("".into());
            
            if !raw_expr.is_empty() {
                match MathEvaluator::evaluate(&raw_expr, &vars_calc) {
                    Ok(val) => {
                        let formatted = format!("{}", val);
                        app.set_result(formatted.clone().into());
                        
                        // Add to history
                        let history_model = hist_calc.add(format!("{} = {}", raw_expr, formatted));
                        app.set_history_list(history_model);
                    }
                    Err(e) => {
                        app.set_syntax_error(e.into());
                    }
                }
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_toggle_history(move || {
        if let Some(app) = app_weak.upgrade() {
            let state = app.get_show_history();
            app.set_show_history(!state);
            app.set_mini_mode(false);
        }
    });

    let app_weak = app.as_weak();
    app.on_toggle_mini_mode(move || {
        if let Some(app) = app_weak.upgrade() {
            let state = app.get_mini_mode();
            let new_state = !state;
            app.set_mini_mode(new_state);
            app.set_show_history(false);
            
            // Adjust window size physically
            if new_state {
                app.window().set_size(slint::PhysicalSize::new(420, 260));
            } else {
                app.window().set_size(slint::PhysicalSize::new(420, 780));
            }
        }
    });

    app.run()
}
