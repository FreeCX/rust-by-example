pub fn calculate(string: &str) -> Vec<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for token in string.trim().split(" ") {
        match token {
            "+" | "-" | "*" | "/" => {
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("[error]: stack is empty!"),
                };
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("[error]: stack is empty!"),
                };
                let result: f64 = match token {
                    "+" => b + a,
                    "-" => b - a,
                    "*" => b * a,
                    "/" => b / a,
                    _ => continue,
                };
                stack.push(result);
            }
            _ => {
                stack.push(match token.parse() {
                    Ok(value) => value,
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
