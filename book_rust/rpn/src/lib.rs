use anyhow::Result;

/// Eval RPN
///
/// # Example
/// ```
/// let src = String::from("1 2 + 3 *");
/// let a = rpn::eval(src).unwrap();
/// assert_eq!(a, 9.0);
///
/// let src = String::from(" 1 1 +  2 / 3 *");
/// let a = rpn::eval(src).unwrap();
/// assert_eq!(a, 3.0);
/// ```

pub fn eval(src: String) -> Result<f64> {
    let mut stack: Vec<f64> = vec![];
    let tokens = src.split_whitespace();
    for token in tokens {
        let t = token.trim();
        match t.parse::<f64>() {
            Ok(v) => {
                stack.push(v);
                continue;
            }
            Err(_) => 0.0,
        };

        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("Undefined operator: {}", t),
        }
    }
    Ok(stack.pop().unwrap_or(0.0))
}
