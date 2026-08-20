//! Reverse Polish Notation (RPN) scientific calculator engine.

/// Evaluates a space-separated RPN expression and returns the numeric result.
///
/// Supported operators:
/// - Binary: `+ - * / ^` (power)
/// - Unary: `sqrt sin cos tan log` (natural log, trig in radians)
pub fn evaluate(expression: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    if tokens.is_empty() {
        return Err("expression is empty".to_string());
    }

    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                let b = pop(&mut stack, token)?;
                let a = pop(&mut stack, token)?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("division by zero".to_string());
                        }
                        a / b
                    }
                    "^" => a.powf(b),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            "sqrt" | "sin" | "cos" | "tan" | "log" => {
                let a = pop(&mut stack, token)?;
                let result = match token {
                    "sqrt" => {
                        if a < 0.0 {
                            return Err("sqrt of negative number".to_string());
                        }
                        a.sqrt()
                    }
                    "sin" => a.sin(),
                    "cos" => a.cos(),
                    "tan" => a.tan(),
                    "log" => {
                        if a <= 0.0 {
                            return Err("log of non-positive number".to_string());
                        }
                        a.ln()
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => match token.parse::<f64>() {
                Ok(n) => stack.push(n),
                Err(_) => return Err(format!("unknown operator or non-numeric token: '{token}'")),
            },
        }
    }

    if stack.len() != 1 {
        return Err(format!(
            "invalid expression: expected 1 value on the stack at the end, found {}",
            stack.len()
        ));
    }

    Ok(stack[0])
}

fn pop(stack: &mut Vec<f64>, op: &str) -> Result<f64, String> {
    stack
        .pop()
        .ok_or_else(|| format!("stack underflow: not enough operands for operator '{op}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn happy_path_simple_addition() {
        let result = evaluate("3 4 +").unwrap();
        assert_close(result, 7.0);
    }

    #[test]
    fn precedence_via_rpn() {
        // (3 + 4) * 2 - 1  in RPN
        let result = evaluate("3 4 + 2 * 1 -").unwrap();
        assert_close(result, 13.0);
    }

    #[test]
    fn precedence_via_rpn_division_and_power() {
        // (2 ^ 3) / (5 - 3) = 8 / 2 = 4
        let result = evaluate("2 3 ^ 5 3 - /").unwrap();
        assert_close(result, 4.0);
    }

    #[test]
    fn all_basic_operators() {
        assert_close(evaluate("1 2 +").unwrap(), 3.0);
        assert_close(evaluate("5 3 -").unwrap(), 2.0);
        assert_close(evaluate("4 5 *").unwrap(), 20.0);
        assert_close(evaluate("10 2 /").unwrap(), 5.0);
        assert_close(evaluate("2 10 ^").unwrap(), 1024.0);
    }

    #[test]
    fn sqrt_operator() {
        assert_close(evaluate("9 sqrt").unwrap(), 3.0);
    }

    #[test]
    fn trig_operators_radians() {
        assert_close(evaluate("0 sin").unwrap(), 0.0);
        assert_close(evaluate("0 cos").unwrap(), 1.0);
        assert_close(evaluate("0 tan").unwrap(), 0.0);
    }

    #[test]
    fn log_operator_natural_log() {
        assert_close(evaluate("1 log").unwrap(), 0.0);
        let e = std::f64::consts::E;
        assert_close(evaluate(&format!("{e} log")).unwrap(), 1.0);
    }

    #[test]
    fn error_stack_underflow() {
        let err = evaluate("1 +").unwrap_err();
        assert!(err.contains("stack underflow"), "got: {err}");
    }

    #[test]
    fn error_division_by_zero() {
        let err = evaluate("5 0 /").unwrap_err();
        assert!(err.contains("division by zero"), "got: {err}");
    }

    #[test]
    fn error_unknown_operator() {
        let err = evaluate("3 4 %").unwrap_err();
        assert!(err.contains("unknown operator"), "got: {err}");
    }

    #[test]
    fn error_non_numeric_token() {
        let err = evaluate("abc 4 +").unwrap_err();
        assert!(err.contains("unknown operator or non-numeric token"), "got: {err}");
    }

    #[test]
    fn error_empty_expression() {
        let err = evaluate("").unwrap_err();
        assert!(err.contains("empty"), "got: {err}");
    }

    #[test]
    fn error_too_many_values_left_on_stack() {
        let err = evaluate("1 2 3 +").unwrap_err();
        assert!(err.contains("invalid expression"), "got: {err}");
    }

    #[test]
    fn error_sqrt_negative() {
        let err = evaluate("-4 sqrt").unwrap_err();
        assert!(err.contains("sqrt of negative number"), "got: {err}");
    }

    #[test]
    fn error_log_non_positive() {
        let err = evaluate("0 log").unwrap_err();
        assert!(err.contains("log of non-positive number"), "got: {err}");
    }
}
