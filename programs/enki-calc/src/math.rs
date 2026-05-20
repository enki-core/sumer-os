use meval::Context;
use crate::variables::Variables;

pub struct MathEvaluator;

impl MathEvaluator {
    pub fn evaluate(raw_expr: &str, vars: &Variables) -> Result<String, String> {
        let mut expr = raw_expr.trim().to_string();

        // Auto-close missing parentheses
        let open_count = expr.chars().filter(|c| *c == '(').count();
        let close_count = expr.chars().filter(|c| *c == ')').count();
        if open_count > close_count {
            for _ in 0..(open_count - close_count) {
                expr.push(')');
            }
        }

        // 1. Trig and advanced operations preprocessing
        expr = expr.replace("sec(", "(1/cos(");
        expr = expr.replace("csc(", "(1/sin(");
        expr = expr.replace("cot(", "(1/tan(");

        // 2. Variable assignments parsing (e.g. x = 5)
        if expr.contains('=') {
            let parts: Vec<&str> = expr.split('=').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim();
                let expr_val = parts[1].trim();

                if var_name == "x" || var_name == "y" {
                    match Self::evaluate_basic(expr_val, vars) {
                        Ok((val, is_imaginary)) => {
                            vars.set(var_name, val);
                            let res_str = if is_imaginary { format!("{}i", val) } else { format!("{}", val) };
                            return Ok(res_str);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            return Err("خطأ: معادلة المتغير غير صحيحة".to_string());
        }

        // 3. Regular evaluation
        match Self::evaluate_basic(&expr, vars) {
            Ok((val, is_imaginary)) => {
                let res_str = if is_imaginary { format!("{}i", val) } else { format!("{}", val) };
                Ok(res_str)
            }
            Err(e) => Err(e),
        }
    }

    fn evaluate_basic(expr: &str, vars: &Variables) -> Result<(f64, bool), String> {
        let mut ctx = Context::new();
        ctx.var("x", vars.get("x"));
        ctx.var("y", vars.get("y"));

        let mut is_imaginary = false;
        let mut cleaned_expr = expr.to_string();

        // Loop to find and evaluate nested or flat "sqrt(" blocks
        while let Some(start_idx) = cleaned_expr.find("sqrt(") {
            let arg_start = start_idx + 5;
            let mut depth = 1;
            let mut end_idx = None;

            for (i, c) in cleaned_expr[arg_start..].char_indices() {
                if c == '(' {
                    depth += 1;
                } else if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        end_idx = Some(arg_start + i);
                        break;
                    }
                }
            }

            if let Some(end_idx) = end_idx {
                let arg_str = &cleaned_expr[arg_start..end_idx];
                match meval::eval_str_with_context(arg_str, &ctx) {
                    Ok(arg_val) => {
                        if arg_val < 0.0 {
                            let positive_val = arg_val.abs();
                            let root = positive_val.sqrt();
                            cleaned_expr.replace_range(start_idx..=end_idx, &root.to_string());
                            is_imaginary = true;
                        } else {
                            let root = arg_val.sqrt();
                            cleaned_expr.replace_range(start_idx..=end_idx, &root.to_string());
                        }
                    }
                    Err(_) => {
                        return Err("خطأ رياضي: تأكد من صحة المعادلة!".to_string());
                    }
                }
            } else {
                break;
            }
        }

        match meval::eval_str_with_context(&cleaned_expr, &ctx) {
            Ok(val) => Ok((val, is_imaginary)),
            Err(_) => Err("خطأ رياضي: تأكد من صحة المعادلة!".to_string()),
        }
    }
}
