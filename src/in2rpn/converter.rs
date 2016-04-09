struct InfixNotation<'a> {
    text: &'a str,
    index: usize,
}

impl<'a> InfixNotation<'a> {
    fn new(string: &'a str) -> InfixNotation<'a> {
        InfixNotation {
            text: string.trim().clone(),
            index: 0,
        }
    }
}

impl<'a> Iterator for InfixNotation<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let start_pos = self.index;
        // end of iter()
        if start_pos >= self.text.len() {
            return None;
        }
        let mut end_pos = self.index;
        // skip first `start_pos` characters
        for block in self.text.chars().skip(start_pos) {
            end_pos += 1;
            // find `end_pos` in self.text index
            match block {
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    end_pos -= 1;
                    break;
                }
                _ => continue,
            }
        }
        // check by zero length string
        if end_pos - start_pos == 0 {
            self.index = end_pos + 1;
        } else {
            self.index = end_pos;
        }
        // and return subtext of `self.text`
        Some(&self.text[start_pos..self.index].trim())
    }
}

fn is_digit(block: &str) -> bool {
    // check first character by digit
    block.chars().nth(0).unwrap().is_digit(10)
}

// operator priority
fn get_priority(operator: &str) -> Option<i8> {
    match operator {
        "(" => Some(-1),
        "*" | "/" => Some(1),
        "+" | "-" => Some(2),
        _ => None,
    }
}

// check the possibility to pull out of the stack
fn can_pop(op1: &str, stack: &Vec<&str>) -> bool {
    if stack.len() == 0 {
        return false;
    }
    let p1 = match get_priority(op1) {
        Some(value) => value,
        None => panic!("[error]: unknown operator '{}'", op1),
    };
    let last = match stack.last() {
        Some(value) => *value,
        None => panic!("[error]: function stack is empty"),
    };
    let p2 = match get_priority(last) {
        Some(value) => value,
        None => panic!("[error]: unknown operator '{}'", last),
    };
    p1 >= 0 && p2 >= 0 && p1 >= p2
}

pub fn in2rpn(input: &str) -> String {
    // result string in RPN notation
    let mut result = String::new();
    let it = InfixNotation::new(input);
    // operator stack
    let mut func: Vec<&str> = Vec::new();
    for item in it {
        if is_digit(item) {
            // push digit to result string
            result.push_str(&format!("{} ", item));
        } else {
            // Shunting-yard main part
            if item == ")" {
                // main priority -- data in ( ... )
                while func.len() > 0 && *func.last().unwrap() != "(" {
                    let function = match func.pop() {
                        Some(value) => value,
                        None => panic!("[error]: function stack is empty"),
                    };
                    result.push_str(&format!("{} ", function));
                }
                func.pop();
            } else {
                // until we can take out of the stack our function (operators)
                while can_pop(item, &func) {
                    let function = match func.pop() {
                        Some(value) => value,
                        None => panic!("[error]: function stack is empty"),
                    };
                    result.push_str(&format!("{} ", function));
                }
                func.push(item);
            }
        }
    }
    // take out all the remaining
    while let Some(item) = func.pop() {
        result.push_str(&format!("{} ", item));
    }
    result
}
