pub fn calculate(string: &str) -> Vec<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for token in string.trim().split(' ') {
        match token {
            "+" | "-" | "*" | "/" => {
                // get 2 value from stack
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("[error]: stack is empty!"),
                };
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("[error]: stack is empty!"),
                };
                // eval one of available operation
                let result: f64 = match token {
                    "+" => b + a,
                    "-" => b - a,
                    "*" => b * a,
                    "/" => b / a,
                    _ => continue,
                };
                // and push result back
                stack.push(result);
            }
            _ => {
                stack.push(match token.parse() {
                    // push number to stack
                    Ok(value) => value,
                    // operation unavailable
                    Err(why) => {
                        println!("{}", why);
                        continue;
                    }
                });
            }
        }
    }
    stack
}
